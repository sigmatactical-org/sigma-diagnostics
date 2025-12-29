/**
 * Main DBC Editor Web Component for can-viewer.
 * Provides full editing capabilities for DBC files.
 */

import type { DbcDto } from './types';
import type { CanFrame } from '../../types';
import { createDefaultDbc, createDefaultMessage } from './types';
import { exportDbcToString, deepClone } from './utils';
import { escapeHtml } from '../../utils/html';
import { events, emitDbcStateChange, emitDbcChanged, type Mdf4ChangedEvent } from '../../events';
import { appStore } from '../../store';
import styles from '../../../styles/can-viewer.css?inline';
import './signals-table';
import './signal-editor';
import './messages-list';
import './message-editor';
import './nodes-editor';
import type { MessagesListElement } from './messages-list';
import type { MessageEditorElement } from './message-editor';
import type { NodesEditorElement } from './nodes-editor';

export interface DbcEditorApi {
  loadDbc(path: string): Promise<DbcDto>;
  saveDbcContent(path: string, content: string): Promise<void>;
  newDbc(): Promise<DbcDto>;
  getDbc(): Promise<DbcDto | null>;
  updateDbc(dbc: DbcDto): Promise<void>;
  getCurrentFile(): Promise<string | null>;
  isDirty(): Promise<boolean>;
  openFileDialog(): Promise<string | null>;
  saveFileDialog(defaultPath?: string): Promise<string | null>;
}

export class DbcEditorComponent extends HTMLElement {
  private api: DbcEditorApi | null = null;
  private dbc: DbcDto = createDefaultDbc();
  private currentFile: string | null = null;
  private isDirty = false;
  private selectedMessageId: number | null = null;
  private selectedMessageExtended = false;
  private isAddingMessage = false;
  private activeTab: 'nodes' | 'messages' | 'preview' | 'version' = 'messages';
  private isEditMode = false;
  private dbcBeforeEdit: DbcDto | null = null;
  private frames: CanFrame[] = [];

  // Bound event handler for cleanup
  private handleMdf4Changed = (event: Mdf4ChangedEvent) => this.onMdf4Changed(event);

  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  connectedCallback() {
    this.render();
    events.on('mdf4:changed', this.handleMdf4Changed);
    // Emit initial state so toolbar has correct state
    this.emitStateChange();
  }

  disconnectedCallback() {
    events.off('mdf4:changed', this.handleMdf4Changed);
  }

  /** Handle MDF4 changed event - fetch frames from store for DLC detection */
  private onMdf4Changed(event: Mdf4ChangedEvent): void {
    if (event.action === 'loaded' || event.action === 'capture-stopped') {
      this.frames = appStore.get().mdf4Frames;
    } else if (event.action === 'cleared') {
      this.frames = [];
    }
  }

  setApi(api: DbcEditorApi) {
    this.api = api;
    this.loadInitialState();
  }

  /** Set loaded frames for DLC auto-detection when creating new messages */
  setFrames(frames: CanFrame[]) {
    this.frames = frames;
  }

  getDbc(): DbcDto {
    return this.dbc;
  }

  hasUnsavedChanges(): boolean {
    return this.isDirty;
  }

  private emitStateChange(): void {
    emitDbcStateChange({
      isDirty: this.isDirty,
      isEditing: this.isEditMode,
      currentFile: this.currentFile,
      messageCount: this.dbc.messages.length,
    });
    // Update global DBC file in app store
    appStore.set({ dbcFile: this.currentFile });
  }

  async loadFile(path: string): Promise<void> {
    if (!this.api) return;
    try {
      this.dbc = await this.api.loadDbc(path);
      this.currentFile = path;
      this.isDirty = false;
      this.selectedMessageId = null;
      this.isAddingMessage = false;
      this.render();
      this.emitStateChange();
      // Emit dbc:changed so consumers can fetch from appStore
      emitDbcChanged({
        action: 'loaded',
        dbcInfo: null,
        filename: path.split('/').pop() || null,
      });
      this.showToast('File loaded successfully', 'success');
    } catch (e) {
      this.showToast(`Failed to load file: ${e}`, 'error');
    }
  }

  private async loadInitialState() {
    if (!this.api) return;
    try {
      const dbc = await this.api.getDbc();
      if (dbc) {
        this.dbc = dbc;
        this.currentFile = await this.api.getCurrentFile();
        this.isDirty = await this.api.isDirty();
        this.render();
        this.emitStateChange();
      }
    } catch {
      // No initial state, that's fine
    }
  }

  private render() {
    if (!this.shadowRoot) return;

    this.shadowRoot.innerHTML = `
      <style>${styles}
        :host {
          display: block;
          position: relative;
          width: 100%;
          height: 100%;
        }
      </style>

      <div class="cv-editor-container">
        <div class="cv-editor-header">
          <div class="cv-tabs">
            <button class="cv-tab ${this.activeTab === 'messages' ? 'active' : ''}" data-tab="messages">
              Messages <span class="cv-tab-badge">${this.dbc.messages.length}</span>
            </button>
            <button class="cv-tab ${this.activeTab === 'nodes' ? 'active' : ''}" data-tab="nodes">
              Nodes <span class="cv-tab-badge">${this.dbc.nodes.length}</span>
            </button>
            <button class="cv-tab ${this.activeTab === 'version' ? 'active' : ''}" data-tab="version">
              Version
            </button>
            <button class="cv-tab ${this.activeTab === 'preview' ? 'active' : ''}" data-tab="preview">
              Preview
            </button>
          </div>
        </div>

        <div class="cv-editor-main">
          ${this.activeTab === 'messages' ? this.renderMessagesTab() : ''}
          ${this.activeTab === 'nodes' ? this.renderNodesTab() : ''}
          ${this.activeTab === 'version' ? this.renderVersionTab() : ''}
          ${this.activeTab === 'preview' ? this.renderPreviewTab() : ''}
        </div>
      </div>
    `;

    this.setupEventListeners();
    this.updateChildComponents();
  }

  private renderMessagesTab(): string {
    const selectedMessage = this.isAddingMessage
      ? null
      : this.dbc.messages.find(
          m => m.id === this.selectedMessageId && m.is_extended === this.selectedMessageExtended
        );

    return `
      <div class="cv-grid responsive">
        <cv-messages-list class="cv-card" id="messagesList">
          <div class="cv-card-header">
            <span class="cv-card-title">Messages <span class="cv-tab-badge">${this.dbc.messages.length}</span></span>
            ${this.isEditMode ? `<button class="cv-btn accent small" id="add-message-btn">+ Add</button>` : ''}
          </div>
        </cv-messages-list>

        <div class="cv-card cv-card-no-header" id="messageDetail">
          ${this.isAddingMessage || selectedMessage ? `
            <cv-message-editor data-edit-mode="${this.isEditMode}"></cv-message-editor>
          ` : `
            <div class="cv-empty-state">
              <div class="cv-empty-state-title">No Message Selected</div>
              <p>Select a message from the list to ${this.isEditMode ? 'edit it, or click "Add" to create a new one' : 'view its details'}.</p>
            </div>
          `}
        </div>
      </div>
    `;
  }

  private renderNodesTab(): string {
    return `
      <div class="cv-grid" style="justify-content: center;">
        <div class="cv-card" style="max-width: 600px;">
          <div class="cv-card-header">
            <span class="cv-card-title">ECU/Node Names</span>
          </div>
          <div class="cv-card-body padded">
            <p class="cv-help-text">
              Define the ECU and node names used in this DBC file. These can be used as message senders and signal receivers.
            </p>
            <cv-nodes-editor></cv-nodes-editor>
          </div>
        </div>
      </div>
    `;
  }

  private renderVersionTab(): string {
    return `
      <div class="cv-grid" style="justify-content: center;">
        <div class="cv-card" style="max-width: 600px;">
          <div class="cv-card-header">
            <span class="cv-card-title">DBC Version</span>
          </div>
          <div class="cv-card-body padded">
            <p class="cv-help-text">
              Set the version string for this DBC file. This appears in the VERSION statement at the top of the file.
            </p>
            <div class="cv-form-group">
              <label class="cv-label">Version</label>
              <input type="text" class="cv-input" style="max-width: 200px" id="dbc-version"
                     value="${this.dbc.version || ''}"
                     placeholder="e.g., 1.0">
            </div>
          </div>
        </div>
      </div>
    `;
  }

  private renderPreviewTab(): string {
    const dbcContent = exportDbcToString(this.dbc);
    return `
      <div class="cv-grid">
        <div class="cv-card">
          <div class="cv-card-header">
            <span class="cv-card-title">DBC File Preview</span>
          </div>
          <div class="cv-preview-content">
            <pre class="cv-preview-text">${escapeHtml(dbcContent)}</pre>
          </div>
        </div>
      </div>
    `;
  }

  private setupEventListeners() {
    if (!this.shadowRoot) return;

    // Tabs
    this.shadowRoot.querySelectorAll('.cv-tab').forEach(tab => {
      tab.addEventListener('click', () => {
        this.activeTab = (tab as HTMLElement).dataset.tab as typeof this.activeTab;
        this.render();
      });
    });

    // Messages tab
    this.shadowRoot.getElementById('add-message-btn')?.addEventListener('click', () => {
      this.isAddingMessage = true;
      this.selectedMessageId = null;
      this.render();
    });

    // Message list selection
    const messagesList = this.shadowRoot.querySelector('cv-messages-list');
    messagesList?.addEventListener('message-select', ((e: CustomEvent) => {
      this.selectedMessageId = e.detail.id;
      this.selectedMessageExtended = e.detail.isExtended;
      this.isAddingMessage = false;
      this.render();
    }) as EventListener);

    // Message editor events
    const messageEditor = this.shadowRoot.querySelector('cv-message-editor');
    messageEditor?.addEventListener('message-edit-done', (() => {
      this.handleSaveMessage();
    }) as EventListener);

    messageEditor?.addEventListener('message-delete-request', (() => {
      this.handleDeleteMessage();
    }) as EventListener);

    messageEditor?.addEventListener('message-edit-cancel', (() => {
      this.isAddingMessage = false;
      this.selectedMessageId = null;
      this.render();
    }) as EventListener);

    // Listen for signal changes (triggers isDirty without requiring message Done click)
    messageEditor?.addEventListener('message-change', ((e: Event) => {
      const customEvent = e as CustomEvent;
      const idx = this.dbc.messages.findIndex(
        m => m.id === this.selectedMessageId && m.is_extended === this.selectedMessageExtended
      );
      if (idx >= 0) {
        this.dbc.messages[idx] = customEvent.detail;
        this.isDirty = true;
        this.syncToBackend();
        this.emitStateChange();
      }
    }) as EventListener);

    // Nodes tab
    const nodesEditor = this.shadowRoot.querySelector('cv-nodes-editor');
    nodesEditor?.addEventListener('nodes-change', ((e: Event) => {
      const customEvent = e as CustomEvent;
      this.dbc.nodes = customEvent.detail.nodes;
      this.isDirty = true;
      this.syncToBackend();
      this.emitStateChange();
      this.render();
    }) as EventListener);

    // Version input
    const versionInput = this.shadowRoot.getElementById('dbc-version') as HTMLInputElement;
    versionInput?.addEventListener('input', async () => {
      this.dbc.version = versionInput.value || null;
      this.isDirty = true;
      await this.syncToBackend();
      this.emitStateChange();
    });
  }

  private updateChildComponents() {
    if (!this.shadowRoot) return;

    // Messages list
    const messagesList = this.shadowRoot.querySelector('cv-messages-list') as MessagesListElement;
    if (messagesList) {
      messagesList.setMessages(this.dbc.messages);
      messagesList.setSelected(this.selectedMessageId, this.selectedMessageExtended);
    }

    // Message editor
    const messageEditor = this.shadowRoot.querySelector('cv-message-editor') as MessageEditorElement;
    if (messageEditor) {
      const selectedMessage = this.isAddingMessage
        ? createDefaultMessage()
        : this.dbc.messages.find(
            m => m.id === this.selectedMessageId && m.is_extended === this.selectedMessageExtended
          ) || null;
      messageEditor.setMessage(selectedMessage, this.isAddingMessage);
      messageEditor.setAvailableNodes(this.dbc.nodes.map(n => n.name));
      messageEditor.setFrames(this.frames);
    }

    // Nodes editor
    const nodesEditor = this.shadowRoot.querySelector('cv-nodes-editor') as NodesEditorElement;
    if (nodesEditor) {
      nodesEditor.setNodes(this.dbc.nodes);
    }
  }

  // Public methods for external control
  setEditMode(editing: boolean): void {
    if (editing && !this.isEditMode) {
      // Entering edit mode - store current state for potential cancel
      this.dbcBeforeEdit = deepClone(this.dbc);
    }
    this.isEditMode = editing;
    if (!editing) {
      this.isAddingMessage = false;
      this.dbcBeforeEdit = null;
    }
    this.render();
    this.emitStateChange();
  }

  cancelEdit(): void {
    if (this.dbcBeforeEdit) {
      this.dbc = this.dbcBeforeEdit;
      this.dbcBeforeEdit = null;
      this.isDirty = false;
      this.syncToBackend();
    }
    this.isEditMode = false;
    this.isAddingMessage = false;
    this.selectedMessageId = null;
    this.render();
    this.emitStateChange();
  }

  getEditMode(): boolean {
    return this.isEditMode;
  }

  getIsDirty(): boolean {
    return this.isDirty;
  }

  getCurrentFile(): string | null {
    return this.currentFile;
  }

  getMessageCount(): number {
    return this.dbc.messages.length;
  }

  async handleNew() {
    if (this.isDirty) {
      if (!confirm('You have unsaved changes. Create a new file anyway?')) {
        return;
      }
    }

    if (this.api) {
      try {
        this.dbc = await this.api.newDbc();
        this.currentFile = null;
        this.isDirty = false;
        this.selectedMessageId = null;
        this.isAddingMessage = false;
        this.render();
        this.emitStateChange();
        emitDbcChanged({ action: 'new', dbcInfo: null, filename: null });
        this.showToast('New DBC created', 'success');
      } catch (e) {
        this.showToast(`Failed to create new DBC: ${e}`, 'error');
      }
    } else {
      this.dbc = createDefaultDbc();
      this.currentFile = null;
      this.isDirty = false;
      this.selectedMessageId = null;
      this.isAddingMessage = false;
      this.render();
      this.emitStateChange();
      emitDbcChanged({ action: 'new', dbcInfo: null, filename: null });
    }
  }

  async handleOpen() {
    if (!this.api) return;

    if (this.isDirty) {
      if (!confirm('You have unsaved changes. Open a different file anyway?')) {
        return;
      }
    }

    try {
      const path = await this.api.openFileDialog();
      if (path) {
        await this.loadFile(path);
      }
    } catch (e) {
      this.showToast(`Failed to open file: ${e}`, 'error');
    }
  }

  async handleSave() {
    if (!this.api) return;

    if (this.currentFile) {
      await this.saveToPath(this.currentFile);
    } else {
      await this.handleSaveAs();
    }
  }

  async handleSaveAs() {
    if (!this.api) return;

    try {
      const path = await this.api.saveFileDialog(this.currentFile || undefined);
      if (path) {
        await this.saveToPath(path);
      }
    } catch (e) {
      this.showToast(`Failed to save file: ${e}`, 'error');
    }
  }

  private async saveToPath(path: string) {
    if (!this.api) return;

    try {
      const content = exportDbcToString(this.dbc);
      await this.api.saveDbcContent(path, content);
      // Reload the DBC in the backend so MDF4/Live Capture use the new version for decoding
      await this.api.loadDbc(path);
      this.currentFile = path;
      this.isDirty = false;
      // Clear edit state since we've saved and synced
      this.isEditMode = false;
      this.isAddingMessage = false;
      this.dbcBeforeEdit = null;
      this.render();
      this.emitStateChange();
      // Emit dbc:changed so consumers can react to saved DBC
      emitDbcChanged({
        action: 'updated',
        dbcInfo: null,
        filename: path.split('/').pop() || null,
      });
      this.showToast('File saved successfully', 'success');
    } catch (e) {
      this.showToast(`Failed to save file: ${e}`, 'error');
    }
  }

  private async handleSaveMessage() {
    const messageEditor = this.shadowRoot?.querySelector('de-message-editor') as MessageEditorElement;
    if (!messageEditor) return;

    const message = messageEditor.getMessage();

    if (!message.name) {
      this.showToast('Message name is required', 'error');
      return;
    }

    if (this.isAddingMessage) {
      // Check for duplicate
      if (this.dbc.messages.some(m => m.id === message.id && m.is_extended === message.is_extended)) {
        this.showToast(`Message with ID ${message.id} already exists`, 'error');
        return;
      }
      this.dbc.messages.push(message);
      this.selectedMessageId = message.id;
      this.selectedMessageExtended = message.is_extended;
    } else {
      // Update existing
      const idx = this.dbc.messages.findIndex(
        m => m.id === this.selectedMessageId && m.is_extended === this.selectedMessageExtended
      );
      if (idx >= 0) {
        // Check if changing ID would create duplicate
        if ((message.id !== this.selectedMessageId || message.is_extended !== this.selectedMessageExtended) &&
            this.dbc.messages.some(m => m.id === message.id && m.is_extended === message.is_extended)) {
          this.showToast(`Message with ID ${message.id} already exists`, 'error');
          return;
        }
        this.dbc.messages[idx] = message;
        this.selectedMessageId = message.id;
        this.selectedMessageExtended = message.is_extended;
      }
    }

    this.isDirty = true;
    this.isAddingMessage = false;
    await this.syncToBackend();
    this.render();
    this.emitStateChange();
    this.showToast('Message saved', 'success');
  }

  private async handleDeleteMessage() {
    if (this.selectedMessageId === null) return;

    this.dbc.messages = this.dbc.messages.filter(
      m => !(m.id === this.selectedMessageId && m.is_extended === this.selectedMessageExtended)
    );

    this.selectedMessageId = null;
    this.isDirty = true;
    await this.syncToBackend();
    this.render();
    this.emitStateChange();
    this.showToast('Message deleted', 'success');
  }

  private async syncToBackend() {
    if (!this.api) return;

    try {
      await this.api.updateDbc(this.dbc);
    } catch (e) {
      console.error('Failed to sync to backend:', e);
    }
  }

  private showToast(message: string, type: 'success' | 'error') {
    const toast = document.createElement('div');
    toast.className = `cv-toast ${type}`;
    toast.textContent = message;
    this.shadowRoot?.appendChild(toast);

    setTimeout(() => {
      toast.remove();
    }, 3000);
  }
}

customElements.define('cv-dbc-editor', DbcEditorComponent);
