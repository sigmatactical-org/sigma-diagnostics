(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const a of document.querySelectorAll('link[rel="modulepreload"]'))s(a);new MutationObserver(a=>{for(const n of a)if(n.type==="childList")for(const r of n.addedNodes)r.tagName==="LINK"&&r.rel==="modulepreload"&&s(r)}).observe(document,{childList:!0,subtree:!0});function t(a){const n={};return a.integrity&&(n.integrity=a.integrity),a.referrerPolicy&&(n.referrerPolicy=a.referrerPolicy),a.crossOrigin==="use-credentials"?n.credentials="include":a.crossOrigin==="anonymous"?n.credentials="omit":n.credentials="same-origin",n}function s(a){if(a.ep)return;a.ep=!0;const n=t(a);fetch(a.href,n)}})();async function re(i,e={},t){return window.__TAURI_INTERNALS__.invoke(i,e,t)}async function Ye(i={}){return typeof i=="object"&&Object.freeze(i),await re("plugin:dialog|open",{options:i})}async function Je(i={}){return typeof i=="object"&&Object.freeze(i),await re("plugin:dialog|save",{options:i})}class et{invoke;listen;constructor(){this.invoke=async()=>{throw new Error("Tauri not initialized")},this.listen=async()=>()=>{}}async init(){const e=window.__TAURI__;if(!e)throw new Error("Tauri API not available");this.invoke=e.core.invoke,this.listen=e.event.listen}async loadDbc(e){return await this.invoke("load_dbc",{path:e})}async clearDbc(){await this.invoke("clear_dbc")}async getDbcInfo(){return await this.invoke("get_dbc_info")}async getDbcPath(){return await this.invoke("get_dbc_path")}async getDbcSpecification(){return await this.invoke("get_dbc_specification")}async decodeFrames(e){return await this.invoke("decode_frames",{frames:e})}async loadMdf4(e){return await this.invoke("load_mdf4",{path:e})}async exportLogs(e,t){return await this.invoke("export_logs",{path:e,frames:t})}async listCanInterfaces(){return await this.invoke("list_can_interfaces")}async startCapture(e,t,s=!1,a){await this.invoke("start_capture",{interface:e,captureFile:t,append:s,filters:a||null})}async stopCapture(){return await this.invoke("stop_capture")}async getInitialFiles(){return await this.invoke("get_initial_files")}async saveDbcContent(e,t){await this.invoke("save_dbc_content",{path:e,content:t})}async updateDbcContent(e){return await this.invoke("update_dbc_content",{content:e})}async openFileDialog(e){const t=await Ye({multiple:!1,filters:e});return Array.isArray(t)?t[0]||null:t}async saveFileDialog(e,t){return await Je({filters:e,defaultPath:t})}onCanFrame(e){const t=this.listen("can-frame",s=>{e(s.payload)});return()=>{t.then(s=>s())}}onDecodedSignal(e){const t=this.listen("decoded-signal",s=>{e(s.payload)});return()=>{t.then(s=>s())}}onDecodeError(e){const t=this.listen("decode-error",s=>{e(s.payload)});return()=>{t.then(s=>s())}}onCaptureError(e){const t=this.listen("capture-error",s=>{e(s.payload)});return()=>{t.then(s=>s())}}onLiveCaptureUpdate(e){const t=this.listen("live-capture-update",s=>{e(s.payload)});return()=>{t.then(s=>s())}}onCaptureFinalized(e){const t=this.listen("capture-finalized",s=>{e(s.payload)});return()=>{t.then(s=>s())}}}const tt="modulepreload",st=function(i){return"/"+i},me={},at=function(e,t,s){let a=Promise.resolve();if(t&&t.length>0){let r=function(p){return Promise.all(p.map(c=>Promise.resolve(c).then(d=>({status:"fulfilled",value:d}),d=>({status:"rejected",reason:d}))))};document.getElementsByTagName("link");const o=document.querySelector("meta[property=csp-nonce]"),l=o?.nonce||o?.getAttribute("nonce");a=r(t.map(p=>{if(p=st(p),p in me)return;me[p]=!0;const c=p.endsWith(".css"),d=c?'[rel="stylesheet"]':"";if(document.querySelector(`link[href="${p}"]${d}`))return;const h=document.createElement("link");if(h.rel=c?"stylesheet":tt,c||(h.as="script"),h.crossOrigin="",h.href=p,l&&h.setAttribute("nonce",l),document.head.appendChild(h),c)return new Promise((u,v)=>{h.addEventListener("load",u),h.addEventListener("error",()=>v(new Error(`Unable to preload CSS for ${p}`)))})}))}function n(r){const o=new Event("vite:preloadError",{cancelable:!0});if(o.payload=r,window.dispatchEvent(o),!o.defaultPrevented)throw r}return a.then(r=>{for(const o of r||[])o.status==="rejected"&&n(o.reason);return e().catch(n)})};function P(i,e){return`0x${e?i.toString(16).toUpperCase().padStart(8,"0"):i.toString(16).toUpperCase().padStart(3,"0")}`}function it(i){return i.map(e=>e.toString(16).toUpperCase().padStart(2,"0")).join(" ")}function nt(i){const e=[];return i.is_extended&&e.push("EXT"),i.is_fd&&e.push("FD"),i.brs&&e.push("BRS"),i.esi&&e.push("ESI"),e.join(", ")||"-"}function rt(i,e=6){return i.toFixed(e)}function fe(i,e=4){return i.toFixed(e)}function $(i){return i.split("/").pop()?.split("\\").pop()||i}function be(i,e,t=!1){const s=i.filter(o=>o.can_id===e&&o.is_extended===t);if(s.length===0)return null;const a=new Map;for(const o of s){const l=a.get(o.dlc)||0;a.set(o.dlc,l+1)}let n=0,r=s[0].dlc;for(const[o,l]of a)l>n&&(n=l,r=o);return r}function I(i){return JSON.parse(JSON.stringify(i))}function w(i,e){return new CustomEvent(i,{detail:e,bubbles:!0,composed:!0})}function g(i){const e=document.createElement("div");return e.textContent=i,e.innerHTML}function lt(i){return{all:i=i||new Map,on:function(e,t){var s=i.get(e);s?s.push(t):i.set(e,[t])},off:function(e,t){var s=i.get(e);s&&(t?s.splice(s.indexOf(t)>>>0,1):i.set(e,[]))},emit:function(e,t){var s=i.get(e);s&&s.slice().map(function(a){a(t)}),(s=i.get("*"))&&s.slice().map(function(a){a(e,t)})}}}const f=lt();function O(i){f.emit("dbc:changed",i)}function ot(i){f.emit("dbc:state-change",i)}function xe(i){f.emit("mdf4:changed",i)}function ct(i){f.emit("frame:selected",i)}function dt(i){f.emit("capture:started",i)}function ht(i){f.emit("capture:stopped",i)}function pt(i){f.emit("live:interfaces-loaded",i)}function Be(i){f.emit("tab:switch",i)}function qe(i){let e={...i};const t=new Set;return{get:()=>e,set(s){e={...e,...s},t.forEach(a=>a(e))},subscribe(s){return t.add(s),()=>t.delete(s)}}}const k=qe({dbcFile:null,mdf4File:null,mdf4Frames:[],mdf4Signals:[]}),B=qe({isCapturing:!1,currentInterface:null,frameCount:0,messageCount:0});function ut(i){const e=i.trim().toUpperCase();return e||null}function vt(i,e){const t=e.split(/\s+/);if(t.length>i.length)return!1;for(let s=0;s<t.length;s++){const a=t[s];if(a==="??"||a==="XX")continue;const n=parseInt(a,16);if(isNaN(n)||i[s]!==n)return!1}return!0}function gt(i){const e=i.trim();return e?e.split(",").map(t=>t.trim()).filter(t=>t.length>0).map(t=>parseInt(t,16)).filter(t=>!isNaN(t)):[]}function mt(i){const e=i.trim().toLowerCase();return e?e.split(",").map(t=>t.trim().toLowerCase()).filter(t=>t.length>0):[]}function ze(i){let e=0;return(i.timeMin!==null&&i.timeMin!==void 0||i.timeMax!==null&&i.timeMax!==void 0)&&e++,i.canIds&&i.canIds.length>0&&e++,i.messages&&i.messages.length>0&&e++,i.signals&&i.signals.length>0&&e++,i.dataPattern&&e++,i.channel&&e++,i.matchStatus&&i.matchStatus!=="all"&&e++,e}function ft(i,e){return i?.messages&&i.messages.find(t=>t.id===e)||null}function bt(i,e,t=null){const s=e.canIds.length>0,a=new Set(e.canIds),n=e.messages.length>0,r=e.signals.length>0;return i.filter(o=>{if(e.timeMin!==null&&o.timestamp<e.timeMin||e.timeMax!==null&&o.timestamp>e.timeMax||s&&!a.has(o.can_id))return!1;if(e.channel){const c=e.channel.toLowerCase();if(!o.channel.toLowerCase().includes(c))return!1}if(e.dataPattern&&!vt(o.data,e.dataPattern))return!1;const l=ft(t,o.can_id),p=l!==null;if(e.matchStatus==="matched"&&!p||e.matchStatus==="unmatched"&&p)return!1;if(n){if(!p)return!1;const c=l.name.toLowerCase();if(!e.messages.some(d=>c.includes(d)))return!1}if(r){if(!p)return!1;const c=l.signals.map(d=>d.name.toLowerCase());if(!e.signals.some(d=>c.some(h=>h.includes(d))))return!1}return!0})}function xt(){return{timeMin:null,timeMax:null,canIds:null,messages:null,signals:null,dataPattern:null,channel:null,matchStatus:"all"}}function yt(i){const e=gt(i);return e.length>0?e:null}function ye(i){const e=mt(i);return e.length>0?e:null}function wt(i,e,t=null){const s={timeMin:e.timeMin,timeMax:e.timeMax,canIds:e.canIds??[],messages:e.messages??[],signals:e.signals??[],dataPattern:e.dataPattern,channel:e.channel,matchStatus:e.matchStatus};return bt(i,s,t)}const we={appName:"CAN Viewer",showDbcTab:!0,showLiveTab:!0,showMdf4Tab:!0,showAboutTab:!0,initialTab:"dbc",autoScroll:!0,maxFrames:1e4,maxSignals:1e4};function kt(i){return"Int"in i?{type:"int",min:i.Int.min,max:i.Int.max}:"Hex"in i?{type:"hex",min:i.Hex.min,max:i.Hex.max}:"Float"in i?{type:"float",min:i.Float.min,max:i.Float.max}:"String"in i?{type:"string"}:"Enum"in i?{type:"enum",values:i.Enum.values}:{type:"string"}}function St(i){return"Network"in i?{type:"network"}:"Node"in i?{type:"node",node_name:i.Node.node_name}:"Message"in i?{type:"message",message_id:i.Message.message_id}:"Signal"in i?{type:"signal",message_id:i.Signal.message_id,signal_name:i.Signal.signal_name}:{type:"network"}}function ke(i){return"Int"in i?i.Int:"Float"in i?i.Float:"String"in i?i.String:0}function Se(i){return{version:i.version||null,bit_timing:i.bit_timing?{baudrate:i.bit_timing.baudrate,btr1:i.bit_timing.btr1,btr2:i.bit_timing.btr2}:null,comment:i.comment||null,nodes:i.nodes.map(e=>({name:e.name,comment:e.comment||null})),messages:i.messages.map(e=>({id:e.id,is_extended:!1,name:e.name,dlc:e.dlc,sender:e.sender||"Vector__XXX",signals:e.signals.map(t=>({name:t.name,start_bit:t.start_bit,length:t.length,byte_order:t.byte_order==="big_endian"?"big_endian":"little_endian",is_unsigned:!t.is_signed,factor:t.factor,offset:t.offset,min:t.min,max:t.max,unit:t.unit||null,receivers:{type:"none"},is_multiplexer:t.is_multiplexer||!1,multiplexer_value:t.multiplexer_value??null,comment:t.comment||null})),comment:e.comment||null})),value_descriptions:i.value_descriptions.map(e=>({message_id:e.message_id,signal_name:e.signal_name,descriptions:e.descriptions.map(t=>({value:t.value,description:t.description}))})),attribute_definitions:i.attribute_definitions.map(e=>({name:e.name,object_type:e.object_type,value_type:kt(e.value_type)})),attribute_defaults:i.attribute_defaults.map(e=>({name:e.name,value:ke(e.value)})),attribute_values:i.attribute_values.map(e=>({name:e.name,target:St(e.target),value:ke(e.value)})),extended_multiplexing:i.extended_multiplexing.map(e=>({message_id:e.message_id,signal_name:e.signal_name,multiplexer_signal:e.multiplexer_signal,ranges:e.ranges}))}}function Et(){return{version:null,bit_timing:null,nodes:[],messages:[],comment:null,value_descriptions:[],attribute_definitions:[],attribute_defaults:[],attribute_values:[],extended_multiplexing:[]}}var Ee;(function(i){i.Nsis="nsis",i.Msi="msi",i.Deb="deb",i.Rpm="rpm",i.AppImage="appimage",i.App="app"})(Ee||(Ee={}));async function $t(){return re("plugin:app|version")}class Ct extends HTMLElement{unsubscribeStore=null;isCapturing=!1;handleCaptureStarted=e=>this.onCaptureStarted();handleCaptureStopped=e=>this.onCaptureStopped();constructor(){super()}connectedCallback(){this.render(),this.bindEvents(),this.unsubscribeStore=k.subscribe(()=>this.updateStatusUI()),f.on("capture:started",this.handleCaptureStarted),f.on("capture:stopped",this.handleCaptureStopped)}disconnectedCallback(){this.unsubscribeStore?.(),f.off("capture:started",this.handleCaptureStarted),f.off("capture:stopped",this.handleCaptureStopped)}onCaptureStarted(){this.isCapturing=!0,this.updateStatusUI()}onCaptureStopped(){this.isCapturing=!1,this.updateStatusUI()}render(){this.className="cv-toolbar cv-tab-pane",this.id="mdf4Tab",this.innerHTML=`
      <button class="cv-btn primary" id="openBtn">Open</button>
      <button class="cv-btn" id="clearBtn">Clear</button>
      <span class="cv-status"><span class="cv-status-dot" id="statusDot"></span><span id="statusText">No file loaded</span></span>
    `}bindEvents(){this.querySelector("#openBtn")?.addEventListener("click",()=>{this.dispatchEvent(w("open",{}))}),this.querySelector("#clearBtn")?.addEventListener("click",()=>{this.dispatchEvent(w("clear",{}))})}updateStatusUI(){const e=this.querySelector("#statusDot"),t=this.querySelector("#statusText"),s=k.get().mdf4File;e?.classList.remove("active","capturing"),this.isCapturing?e?.classList.add("capturing"):s&&e?.classList.add("active"),t&&(this.isCapturing&&s?t.textContent=`Capturing to ${$(s)}...`:s?t.textContent=$(s):t.textContent="No file loaded")}}customElements.define("cv-mdf4-toolbar",Ct);class _t extends HTMLElement{unsubscribeStore=null;handleInterfacesLoaded=e=>this.onInterfacesLoaded(e);connectedCallback(){this.render(),this.bindEvents(),f.on("live:interfaces-loaded",this.handleInterfacesLoaded),this.unsubscribeStore=B.subscribe(e=>this.onStoreChange(e))}disconnectedCallback(){f.off("live:interfaces-loaded",this.handleInterfacesLoaded),this.unsubscribeStore?.()}onInterfacesLoaded(e){const t=this.querySelector("#interfaceSelect");t&&(t.innerHTML='<option value="">Select CAN interface...</option>'+e.interfaces.map(s=>`<option value="${s}">${s}</option>`).join("")),this.updateButtonStates(B.get())}onStoreChange(e){this.updateStatusUI(e),this.updateButtonStates(e)}render(){this.className="cv-toolbar cv-tab-pane",this.id="liveTab",this.innerHTML=`
      <label>Interface:</label>
      <select class="cv-select" id="interfaceSelect">
        <option value="">Select CAN interface...</option>
      </select>
      <button class="cv-btn" id="refreshBtn">↻</button>
      <span class="cv-toolbar-sep"></span>
      <button class="cv-btn success" id="startBtn" disabled>Start</button>
      <button class="cv-btn danger" id="stopBtn" disabled>Stop</button>
      <button class="cv-btn" id="clearBtn">Clear</button>
      <span class="cv-status"><span class="cv-status-dot" id="statusDot"></span><span id="statusText">Idle</span></span>
    `}bindEvents(){this.querySelector("#refreshBtn")?.addEventListener("click",()=>{this.dispatchEvent(w("refresh-interfaces",{}))}),this.querySelector("#startBtn")?.addEventListener("click",()=>{const e=this.querySelector("#interfaceSelect");e?.value&&this.dispatchEvent(w("start-capture",{interface:e.value}))}),this.querySelector("#stopBtn")?.addEventListener("click",()=>{this.dispatchEvent(w("stop-capture",{}))}),this.querySelector("#clearBtn")?.addEventListener("click",()=>{this.dispatchEvent(w("clear",{}))}),this.querySelector("#interfaceSelect")?.addEventListener("change",()=>{this.updateButtonStates(B.get())})}updateStatusUI(e){const t=this.querySelector("#statusDot"),s=this.querySelector("#statusText"),a=k.get().mdf4File;t?.classList.toggle("active",e.isCapturing),s&&(a?s.textContent=$(a):s.textContent=e.isCapturing?"Capturing...":"Idle")}updateButtonStates(e){const t=this.querySelector("#interfaceSelect"),s=this.querySelector("#startBtn"),a=this.querySelector("#stopBtn");s&&t&&(s.disabled=!t.value||e.isCapturing),a&&(a.disabled=!e.isCapturing)}}customElements.define("cv-live-toolbar",_t);class Mt extends HTMLElement{isDirty=!1;isEditing=!1;messageCount=0;unsubscribeStore=null;handleStateChange=e=>this.onStateChange(e);constructor(){super()}connectedCallback(){this.render(),this.bindEvents(),f.on("dbc:state-change",this.handleStateChange),this.unsubscribeStore=k.subscribe(()=>this.updateStatusUI())}disconnectedCallback(){f.off("dbc:state-change",this.handleStateChange),this.unsubscribeStore?.()}onStateChange(e){this.isDirty=e.isDirty,this.isEditing=e.isEditing,this.messageCount=e.messageCount,this.updateUI()}render(){this.className="cv-toolbar cv-tab-pane",this.id="dbcTab",this.innerHTML=`
      <button class="cv-btn" id="newBtn">New</button>
      <button class="cv-btn" id="openBtn">Open</button>
      <button class="cv-btn primary" id="editBtn">Edit</button>
      <button class="cv-btn" id="cancelBtn" style="display:none">Cancel</button>
      <button class="cv-btn" id="saveBtn" disabled>Save</button>
      <button class="cv-btn" id="saveAsBtn" disabled>Save As</button>
      <span class="cv-status"><span class="cv-status-dot" id="statusDot"></span><span id="statusText">No file loaded</span></span>
    `}bindEvents(){this.querySelector("#newBtn")?.addEventListener("click",()=>{this.dispatchEvent(w("new",{}))}),this.querySelector("#openBtn")?.addEventListener("click",()=>{this.dispatchEvent(w("open",{}))}),this.querySelector("#editBtn")?.addEventListener("click",()=>{this.dispatchEvent(w("edit",{}))}),this.querySelector("#cancelBtn")?.addEventListener("click",()=>{this.dispatchEvent(w("cancel",{}))}),this.querySelector("#saveBtn")?.addEventListener("click",()=>{this.dispatchEvent(w("save",{}))}),this.querySelector("#saveAsBtn")?.addEventListener("click",()=>{this.dispatchEvent(w("save-as",{}))})}updateUI(){const e=this.querySelector("#editBtn"),t=this.querySelector("#cancelBtn"),s=this.querySelector("#saveBtn"),a=this.querySelector("#saveAsBtn");e&&(e.style.display=this.isEditing?"none":""),t&&(t.style.display=this.isEditing?"":"none"),s&&(s.disabled=!this.isDirty,s.classList.toggle("success",this.isDirty)),a&&(a.disabled=this.messageCount===0),this.updateStatusUI()}updateStatusUI(){const e=this.querySelector("#statusDot"),t=this.querySelector("#statusText"),s=k.get().dbcFile;if(e?.classList.toggle("active",!!s&&!this.isDirty),e?.classList.toggle("warning",this.isDirty),t){const a=s?$(s):"No file loaded";t.textContent=this.isDirty?`${a} *`:a}}}customElements.define("cv-dbc-toolbar",Mt);class It extends HTMLElement{unsubscribeStore=null;isDirty=!1;handleStateChange=e=>this.onStateChange(e);connectedCallback(){this.render(),this.unsubscribeStore=k.subscribe(()=>this.updateUI()),f.on("dbc:state-change",this.handleStateChange)}disconnectedCallback(){this.unsubscribeStore?.(),f.off("dbc:state-change",this.handleStateChange)}onStateChange(e){this.isDirty=e.isDirty,this.updateUI()}render(){this.className="cv-stat clickable",this.innerHTML=`
      <span class="cv-stat-label">DBC</span>
      <span class="cv-stat-value">No file loaded</span>
    `,this.addEventListener("click",()=>{Be({tab:"dbc"})})}updateUI(){const{dbcFile:e}=k.get(),t=this.querySelector(".cv-stat-value");if(this.classList.remove("success","warning"),e&&!this.isDirty?this.classList.add("success"):e&&this.isDirty&&this.classList.add("warning"),t)if(e){const s=$(e);t.textContent=this.isDirty?`${s} *`:s,t.setAttribute("title",e)}else t.textContent="No file loaded",t.removeAttribute("title")}}customElements.define("cv-dbc-status",It);class Lt extends HTMLElement{unsubscribeAppStore=null;unsubscribeLiveStore=null;connectedCallback(){this.render(),this.unsubscribeAppStore=k.subscribe(()=>this.updateUI()),this.unsubscribeLiveStore=B.subscribe(()=>this.updateUI())}disconnectedCallback(){this.unsubscribeAppStore?.(),this.unsubscribeLiveStore?.()}render(){this.className="cv-stat clickable",this.innerHTML=`
      <span class="cv-stat-label">MDF4</span>
      <span class="cv-stat-value">No file loaded</span>
    `,this.addEventListener("click",()=>{const{isCapturing:e}=B.get();Be({tab:e?"live":"mdf4"})})}updateUI(){const{mdf4File:e}=k.get(),{isCapturing:t}=B.get(),s=this.querySelector(".cv-stat-value");this.classList.remove("success","warning"),t?(this.classList.add("warning"),s&&(s.textContent="Capturing...",s.removeAttribute("title"))):e?(this.classList.add("success"),s&&(s.textContent=$(e),s.setAttribute("title",e))):s&&(s.textContent="No file loaded",s.removeAttribute("title"))}}customElements.define("cv-mdf4-status",Lt);const L=':host{--cv-bg: #0a0a0a;--cv-bg-alt: #111;--cv-bg-elevated: #1a1a1a;--cv-text: #ccc;--cv-text-muted: #666;--cv-text-dim: #444;--cv-border: #222;--cv-radius: 4px;--cv-accent: #3b82f6;--cv-accent-rgb: 59, 130, 246;--cv-success: #22c55e;--cv-danger: #ef4444;--cv-warning: #eab308;--cv-font-mono: ui-monospace, "Cascadia Code", Consolas, monospace;--cv-font-sans: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;display:flex;flex-direction:column;flex:1;min-height:0;overflow:hidden;font-family:var(--cv-font-sans);background:var(--cv-bg);color:var(--cv-text);color-scheme:dark}*{margin:0;padding:0;box-sizing:border-box}.hidden{display:none!important}.cv-card,.cv-panel{display:flex;flex-direction:column;background:var(--cv-bg-alt);border:1px solid var(--cv-border);border-radius:var(--cv-radius);overflow:hidden}.cv-card{flex:1}.cv-panel{height:100%;font-size:14px}.cv-card-header,.cv-panel-header{display:flex;align-items:center;border-bottom:1px solid var(--cv-border)}.cv-card-header{justify-content:space-between;padding:8px 12px;background:var(--cv-bg-elevated)}.cv-panel-header{padding:0;background:var(--cv-bg-alt)}.cv-card-title{font-size:.85rem;font-weight:500;color:var(--cv-text-muted);text-transform:uppercase;letter-spacing:.5px}.cv-card-no-header{border-top:none}.cv-msg-card-header{flex-direction:column;align-items:stretch;padding:12px;gap:4px}.cv-msg-card-header .cv-bit-layout{margin-top:8px}.cv-card-body,.cv-panel-body{flex:1}.cv-card-body{padding:12px 16px;color:var(--cv-text-muted);font-size:.85rem;line-height:1.5;margin:0}.cv-panel-body{padding:8px}.cv-panel-body.flush{padding:0}.cv-live-viewer,.cv-mdf4-inspector{display:flex;flex-direction:column;flex:1;min-height:0;overflow:hidden}.cv-live-viewer>.cv-panel,.cv-mdf4-inspector>.cv-panel{display:flex;flex-direction:column;flex:1;min-height:0;overflow:hidden}.cv-live-viewer>.cv-panel>.cv-panel-body,.cv-mdf4-inspector>.cv-panel>.cv-panel-body{display:flex;flex-direction:column;flex:1;min-height:0;overflow:hidden}.cv-live-viewer .cv-tab-pane,.cv-mdf4-inspector .cv-tab-pane{display:none}.cv-live-viewer .cv-tab-pane.active,.cv-mdf4-inspector .cv-tab-pane.active{display:flex;flex-direction:column;flex:1;min-height:0;overflow:hidden}.cv-mdf4-inspector .cv-grid,.cv-editor-main .cv-grid{flex:1;min-height:0;overflow:hidden}.cv-mdf4-inspector .cv-card,.cv-editor-main .cv-card{display:flex;flex-direction:column;min-height:0;overflow:hidden}.cv-editor-main .cv-card>cv-message-editor{flex:1;min-height:0;overflow-y:auto}.cv-editor-main cv-messages-list.cv-card{flex:0 0 280px;min-width:200px;max-width:320px}.cv-editor-main #messageDetail{flex:1;min-width:0}.cv-live-viewer .cv-table-wrap,.cv-mdf4-inspector .cv-table-wrap,.cv-editor-main .cv-table-wrap{flex:1;min-height:0;overflow-y:auto}.cv-table-wrap::-webkit-scrollbar,.cv-sidebar-content::-webkit-scrollbar,.cv-detail::-webkit-scrollbar,.cv-signal-editor-panel::-webkit-scrollbar,.cv-markdown-content::-webkit-scrollbar{width:8px}.cv-table-wrap::-webkit-scrollbar-track,.cv-sidebar-content::-webkit-scrollbar-track,.cv-detail::-webkit-scrollbar-track,.cv-signal-editor-panel::-webkit-scrollbar-track,.cv-markdown-content::-webkit-scrollbar-track{background:var(--cv-bg)}.cv-table-wrap::-webkit-scrollbar-thumb,.cv-sidebar-content::-webkit-scrollbar-thumb,.cv-detail::-webkit-scrollbar-thumb,.cv-signal-editor-panel::-webkit-scrollbar-thumb,.cv-markdown-content::-webkit-scrollbar-thumb{background:var(--cv-border);border-radius:4px}.cv-table-wrap::-webkit-scrollbar-thumb:hover,.cv-sidebar-content::-webkit-scrollbar-thumb:hover,.cv-detail::-webkit-scrollbar-thumb:hover,.cv-signal-editor-panel::-webkit-scrollbar-thumb:hover,.cv-markdown-content::-webkit-scrollbar-thumb:hover{background:var(--cv-text-muted)}.cv-stats-bar{display:flex;gap:24px;padding:8px 16px;background:var(--cv-bg-elevated);border-top:1px solid var(--cv-border)}.cv-tabs{display:flex}.cv-tabs.bordered{border-bottom:1px solid var(--cv-border);margin-bottom:15px}.cv-tab{padding:10px 20px;border:none;border-bottom:2px solid transparent;background:transparent;color:var(--cv-text-muted);font-size:.9rem;font-weight:500;cursor:pointer;transition:all .15s}.cv-tab:hover{color:var(--cv-text);background:var(--cv-bg-elevated)}.cv-tab.active{color:var(--cv-accent);border-bottom-color:var(--cv-accent)}.cv-tab-badge{font-size:.75rem;color:var(--cv-text-dim);margin-left:6px;padding:2px 6px;background:var(--cv-bg);border-radius:3px}.cv-tab-badge.dimmed{opacity:.5}.cv-tab-badge.active{background:#3b82f640;color:var(--cv-accent);opacity:1}.cv-tab-badge-error:not(:empty):not([data-count="0"]){background:#ef444433;color:var(--cv-danger)}.cv-tab.active .cv-tab-badge{background:#3b82f633;color:var(--cv-accent)}.cv-tab.active .cv-tab-badge-error:not(:empty):not([data-count="0"]){background:#ef44444d;color:var(--cv-danger)}.cv-tab.active .cv-tab-badge.dimmed{opacity:.6}.cv-tab-pane{display:none!important}.cv-tab-pane.active{display:block!important}.cv-toolbar.cv-tab-pane.active{display:flex!important}.cv-grid{display:flex;height:100%;padding:8px;gap:8px}@media(max-width:1200px){.cv-grid.responsive{flex-direction:column}}.cv-app{display:flex;flex-direction:column;height:100%;max-width:1800px;margin:0 auto;padding:20px;overflow:hidden}.cv-app>.cv-panel,.cv-app>cv-mdf4-inspector,.cv-app>cv-live-viewer,.cv-app>cv-dbc-editor{display:flex;flex-direction:column;flex:1;min-height:0;overflow:hidden}.cv-app-header{flex-shrink:0;background:var(--cv-bg-alt);padding:15px 12px;border-radius:var(--cv-radius);margin-bottom:20px;box-shadow:0 0 0 1px var(--cv-border)}.cv-app-header-top{display:flex;align-items:center;justify-content:space-between;margin-bottom:15px}.cv-header-row{display:flex;align-items:center;justify-content:space-between;gap:16px;flex-wrap:wrap}.cv-header-status{display:flex;gap:8px}.cv-app-title{font-size:1.2rem;color:var(--cv-text-muted);font-weight:500;margin:0}.cv-stats{display:flex;gap:16px;margin-left:auto;margin-right:16px}.cv-stat{display:flex;flex-direction:column;align-items:center;padding:4px 10px;background:var(--cv-bg-alt);border:1px solid var(--cv-border);border-radius:var(--cv-radius);min-width:60px;font-size:.85rem;color:var(--cv-text-muted)}.cv-stat.clickable{cursor:pointer;transition:all .15s}.cv-stat.clickable:hover{background:var(--cv-bg-elevated)}.cv-stat.success{border-color:#22c55e4d;color:var(--cv-success)}.cv-stat.success:hover{background:#22c55e1a}.cv-stat.danger{border-color:#ef44444d;color:var(--cv-danger)}.cv-stat.danger:hover{background:#ef44441a}.cv-stat.warning{border-color:#eab3084d;color:var(--cv-warning)}.cv-stat.warning:hover{background:#eab3081a}.cv-stat-label{font-size:.65rem;color:var(--cv-text-dim);text-transform:uppercase;letter-spacing:.5px}.cv-toolbar,.cv-filters{display:flex;flex-wrap:wrap;gap:15px;align-items:center}.cv-filters{padding:12px 16px}.cv-filters-grid{display:grid;grid-template-columns:1fr 1fr auto;gap:24px;padding:16px}.cv-filter-section{display:flex;flex-direction:column;gap:10px}.cv-filter-section-title{font-size:.75rem;font-weight:600;color:var(--cv-text-muted);text-transform:uppercase;letter-spacing:.5px;padding-bottom:6px;border-bottom:1px solid var(--cv-border)}.cv-filter-row{display:flex;align-items:center;gap:8px}.cv-filter-row label{font-size:.8rem;color:var(--cv-text-muted);min-width:90px}.cv-filter-row .cv-input,.cv-filter-row .cv-select{flex:1}.cv-filter-sep{color:var(--cv-text-dim);font-size:.8rem}.cv-filter-actions{display:flex;flex-direction:column;justify-content:space-between;min-width:160px}.cv-filter-summary{font-size:.8rem;color:var(--cv-text-muted);padding:8px;background:var(--cv-bg);border-radius:var(--cv-radius);text-align:center}.cv-filter-link{font-size:.75rem;color:var(--cv-text-dim);cursor:pointer;transition:color .15s}.cv-filter-link:hover,.cv-filter-link.active{color:var(--cv-accent)}.cv-filter-chips{display:flex;flex-wrap:wrap;gap:6px;padding:0 12px 8px}.cv-filter-chips:empty{display:none}.cv-filter-chip{display:inline-flex;align-items:center;gap:4px;padding:2px 6px 2px 8px;background:var(--cv-accent);color:var(--cv-bg);border-radius:12px;font-size:.75rem;font-weight:500}.cv-filter-chip-x{display:inline-flex;align-items:center;justify-content:center;width:16px;height:16px;padding:0;border:none;border-radius:50%;background:#0003;color:inherit;font-size:14px;line-height:1;cursor:pointer}.cv-filter-chip-x:hover{background:#0006}@media(max-width:1000px){.cv-filters-grid{grid-template-columns:1fr}}.cv-toolbar>label,.cv-filter-group label{font-size:.85rem;color:var(--cv-text-muted);white-space:nowrap}.cv-btn{height:28px;padding:0 12px;border:1px solid var(--cv-border);border-radius:3px;cursor:pointer;font-size:.8rem;transition:all .15s;background:var(--cv-bg-alt);color:var(--cv-text-muted)}.cv-btn:hover:not(:disabled){background:var(--cv-bg-elevated);color:var(--cv-text)}.cv-btn:disabled{opacity:.3;cursor:not-allowed}.cv-btn.small{height:22px;padding:0 10px;font-size:.75rem}.cv-btn.primary{background:var(--cv-border);color:var(--cv-text)}.cv-btn.primary:hover:not(:disabled){background:var(--cv-bg-elevated)}.cv-btn.accent{background:var(--cv-accent);color:#fff;border-color:var(--cv-accent)}.cv-btn.accent:hover:not(:disabled){filter:brightness(1.15)}.cv-btn.success{background:var(--cv-success);color:#fff;border-color:var(--cv-success)}.cv-btn.danger{background:var(--cv-danger);color:#fff;border-color:var(--cv-danger)}.cv-btn.success:hover:not(:disabled),.cv-btn.danger:hover:not(:disabled){filter:brightness(1.1)}.cv-btn.ghost{background:var(--cv-bg);color:var(--cv-text)}.cv-btn.ghost:hover:not(:disabled){background:var(--cv-bg-alt)}.cv-input,.cv-select{height:28px;padding:0 10px;border:1px solid var(--cv-border);border-radius:3px;background:var(--cv-bg);color:var(--cv-text-muted);font-size:.8rem}.cv-input:focus,.cv-select:focus{outline:1px solid var(--cv-accent);outline-offset:-1px}.cv-input.mono{font-family:var(--cv-font-mono);color:var(--cv-text)}.cv-input::placeholder{color:var(--cv-text-dim)}.cv-input:not(:placeholder-shown){color:var(--cv-text)}.cv-select.has-value{color:var(--cv-text)}.cv-input.wide{width:150px}.cv-input.filter-active,.cv-select.filter-active{border-color:var(--cv-accent);background:color-mix(in srgb,var(--cv-accent) 10%,var(--cv-bg))}.cv-input.filter-confirm,.cv-select.filter-confirm{background:color-mix(in srgb,var(--cv-success) 25%,var(--cv-bg));transition:background .15s ease-out}#interfaceSelect{width:180px}.cv-select option{font-weight:700;color:var(--cv-text)}.cv-select option[value=""]{font-weight:400;color:var(--cv-text-muted)}.cv-status{display:flex;align-items:center;gap:8px;font-size:.85rem;color:var(--cv-text-muted)}.cv-status-dot{width:8px;height:8px;border-radius:50%;background:var(--cv-border)}.cv-status-dot.active{background:var(--cv-success);box-shadow:0 0 6px var(--cv-success)}.cv-status-dot.warning,.cv-status-dot.capturing{background:var(--cv-warning);box-shadow:0 0 6px var(--cv-warning)}.cv-status-dot.pulse{animation:cv-pulse 1.5s ease-in-out infinite}.cv-stat-card{background:var(--cv-bg-alt);border:1px solid var(--cv-border);border-radius:var(--cv-radius);padding:12px 16px}.cv-stat-card .cv-stat-value{font-size:1.5rem;font-weight:600;font-family:var(--cv-font-mono);color:var(--cv-text)}.cv-stat-card .cv-stat-label{font-size:.7rem;color:var(--cv-text-dim);text-transform:uppercase;letter-spacing:.5px;margin-top:4px}.cv-section-title-sm{font-size:.7rem;font-weight:600;color:var(--cv-text-dim);text-transform:uppercase;letter-spacing:.5px;margin-bottom:8px}.cv-toolbar-stat{display:flex;align-items:baseline;gap:4px;padding:0 8px;border-left:1px solid var(--cv-border)}.cv-toolbar-stat-value{font-family:var(--cv-font-mono);font-size:.85rem;font-weight:600;color:var(--cv-accent)}.cv-toolbar-stat-label{font-size:.7rem;color:var(--cv-text-dim);text-transform:uppercase}.cv-toolbar-sep{width:1px;height:20px;background:var(--cv-border);margin:0 4px}.cv-toolbar[data-disabled=true]{opacity:.5;pointer-events:none}.cv-tab.disabled{opacity:.5;cursor:not-allowed}.cv-table-wrap{height:100%;overflow-y:auto}.cv-table{width:100%;border-collapse:collapse;font-size:.85rem}.cv-table th{position:sticky;top:0;background:var(--cv-bg-alt);padding:8px 12px;text-align:left;font-weight:500;font-size:.75rem;color:var(--cv-text-muted);text-transform:uppercase;letter-spacing:.5px;border-bottom:1px solid var(--cv-border)}.cv-table td{padding:8px 12px;border-bottom:1px solid var(--cv-border);font-family:var(--cv-font-mono);height:36px}.cv-table tr:hover{background:var(--cv-bg-elevated)}.cv-table tr.clickable{cursor:pointer}.cv-table tr.matched{border-left:3px solid var(--cv-accent)}.cv-table tr.selected{background:#3b82f626!important}.cv-cell-dim{color:var(--cv-text-dim)}.cv-cell-id{color:var(--cv-text);font-weight:600}.cv-cell-name{color:var(--cv-accent);font-weight:500}.cv-cell-nomatch{color:var(--cv-danger)}.cv-cell-data{color:var(--cv-text-muted);letter-spacing:1px}.cv-cell-value{color:var(--cv-text);font-weight:600;text-align:right;font-variant-numeric:tabular-nums}.cv-cell-unit{color:var(--cv-text-muted);font-style:italic}.cv-table-compact th,.cv-table-compact td{padding:6px 8px}.cv-table-compact td{font-family:inherit;height:auto}.cv-signal-description{font-size:.75rem;color:var(--cv-text-dim);font-style:italic;margin-top:2px}.cv-signal-monitor{display:flex;flex-direction:column;gap:2px;padding:8px;overflow-y:auto}.cv-signal-group-header{background:var(--cv-bg-alt);color:var(--cv-text);font-weight:600;font-size:.85rem;padding:8px 12px;margin-top:8px;border-radius:var(--cv-radius)}.cv-signal-group-header:first-child{margin-top:0}.cv-signal-row{display:grid;grid-template-columns:160px 1fr;gap:16px;align-items:center;padding:8px 12px;background:var(--cv-bg);border-radius:var(--cv-radius)}.cv-signal-row:hover{background:var(--cv-bg-elevated)}.cv-signal-info{display:flex;flex-direction:column;gap:2px;min-width:0;flex-shrink:0}.cv-signal-name{font-size:.85rem;color:var(--cv-text);font-weight:500;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}.cv-signal-value{font-size:1rem;font-weight:600;color:var(--cv-accent);font-family:var(--cv-font-mono);font-variant-numeric:tabular-nums;white-space:nowrap}.cv-signal-unit{font-size:.8rem;font-weight:400;color:var(--cv-text-muted)}.cv-signal-chart{display:flex;align-items:center;gap:8px;flex:1;min-width:0}.cv-signal-min,.cv-signal-max{font-size:.7rem;color:var(--cv-text-dim);font-family:var(--cv-font-mono);font-variant-numeric:tabular-nums;min-width:45px;flex-shrink:0}.cv-signal-min{text-align:right}.cv-signal-max{text-align:left}.cv-value-desc{font-weight:600;color:var(--cv-accent)}.cv-value-num{font-size:.85em;color:var(--cv-text-muted);font-family:var(--cv-font-mono)}.cv-sparkline{flex:1;height:32px;min-width:80px;background:var(--cv-bg-alt);border-radius:4px;overflow:hidden}.cv-sparkline svg{width:100%;height:100%}.cv-group-header td{background:var(--cv-bg-alt);color:var(--cv-text);font-weight:600;padding:6px 12px;border-top:1px solid var(--cv-border)}.cv-group-header:first-child td{border-top:none}.cv-cell-signal{color:var(--cv-accent);font-weight:500;padding-left:32px}.cv-empty{color:var(--cv-text-dim);text-align:center;padding:20px}.cv-cell-error-type{color:var(--cv-danger);font-weight:500}.cv-error-summary-row{background:#ef44440d}.cv-error-summary-row:hover{background:#ef44441a}.cv-decode-error{color:var(--cv-danger);font-size:.8rem;margin-left:auto;padding:0 8px}.cv-list-item{padding:8px 12px;border-bottom:1px solid var(--cv-border);cursor:pointer;transition:background .15s}.cv-list-item:hover{background:var(--cv-bg-elevated)}.cv-list-item.selected{background:var(--cv-bg-elevated);border-left:3px solid var(--cv-accent)}.cv-list-item-title{font-weight:500;color:var(--cv-text);font-size:.9rem}.cv-list-item-subtitle{font-family:var(--cv-font-mono);color:var(--cv-text-muted);font-size:.8rem}.cv-list-item-meta{font-size:.75rem;color:var(--cv-text-dim);margin-top:2px}.cv-list-item-comment{font-size:.75rem;color:var(--cv-text-muted);font-style:italic;margin-top:4px;padding-top:4px;border-top:1px dashed var(--cv-border)}.cv-signal-card{background:var(--cv-bg);border:1px solid var(--cv-border);border-radius:var(--cv-radius);padding:12px;margin-bottom:10px}.cv-signal-card-title{font-weight:500;color:var(--cv-text);font-size:.9rem;margin-bottom:8px}.cv-signal-card-comment{font-size:.75rem;color:var(--cv-text-muted);font-style:italic;margin-bottom:8px;padding-bottom:8px;border-bottom:1px dashed var(--cv-border)}.cv-msg-comment{font-size:.8rem;color:var(--cv-text-muted);font-style:italic;margin-top:8px;padding:8px;background:var(--cv-bg-elevated);border-radius:var(--cv-radius);border-left:3px solid var(--cv-accent)}.cv-msg-comment-inline,.cv-msg-comment-compact{font-size:.8rem;color:var(--cv-text-dim);font-style:italic}.cv-msg-comment-inline{margin-left:8px}.cv-signal-comment{font-size:.75rem;color:var(--cv-text-muted);font-style:italic;margin-bottom:8px;padding:6px 8px;background:var(--cv-bg-elevated);border-radius:var(--cv-radius);border-left:2px solid var(--cv-accent)}.cv-textarea{width:100%;padding:8px 10px;background:var(--cv-input-bg);border:1px solid var(--cv-border);border-radius:var(--cv-radius);color:var(--cv-text);font-size:.85rem;font-family:inherit;resize:vertical;min-height:60px}.cv-textarea:focus{outline:none;border-color:var(--cv-accent)}.cv-signal-props{display:grid;grid-template-columns:repeat(auto-fill,minmax(120px,1fr));gap:8px;font-size:.8rem}.cv-signal-prop{display:flex;flex-direction:column}.cv-signal-prop-label{color:var(--cv-text-dim);font-size:.7rem;text-transform:uppercase}.cv-signal-prop-value{color:var(--cv-text-muted);font-family:var(--cv-font-mono)}.cv-detail-title{font-size:1rem;font-weight:500;color:var(--cv-text)}.cv-detail-subtitle{font-size:.8rem;color:var(--cv-text-muted);margin-top:4px}.cv-about-header{border-bottom:none;align-items:center;gap:32px}#aboutFeatures .cv-grid{display:grid;grid-template-columns:repeat(6,1fr);gap:12px;padding:16px}@media(max-width:1400px){#aboutFeatures .cv-grid{grid-template-columns:repeat(4,1fr)}}@media(max-width:1000px){#aboutFeatures .cv-grid{grid-template-columns:repeat(3,1fr)}}@media(max-width:700px){#aboutFeatures .cv-grid{grid-template-columns:repeat(2,1fr)}}#aboutFeatures .cv-card{border-left:3px solid var(--cv-accent)}#aboutFeatures .cv-card-pro{border-left:3px solid var(--cv-warning)}#aboutFeatures .cv-card-pro .cv-card-title:after{content:" Pro";font-size:.7em;color:var(--cv-warning);font-weight:400}#aboutAcknowledgments .cv-card{border-left:3px solid var(--cv-success)}.cv-about-title-group{display:flex;flex-direction:column;gap:2px}.cv-about-title{font-size:1.2rem;font-weight:600;color:var(--cv-text)}.cv-about-version{color:var(--cv-text-dim);font-size:.8rem}.cv-about-desc{flex:1;color:var(--cv-text-muted);font-size:.85rem;line-height:1.5}.cv-deps-list{list-style:none;margin:0}.cv-deps-list li{padding:8px 0;border-bottom:1px solid var(--cv-border)}.cv-deps-list li:last-child{border-bottom:none}.cv-deps-list li a{color:var(--cv-text);text-decoration:none}.cv-deps-list li a:hover{color:var(--cv-accent)}.cv-deps-list li.cv-sister-project{background:linear-gradient(90deg,rgba(var(--cv-accent-rgb),.08) 0%,transparent 100%);margin:0 -16px;padding:8px 16px;border-radius:4px;border-bottom-color:transparent}.cv-deps-list li.cv-sister-project a{color:var(--cv-accent);font-weight:600}.cv-about-license{color:var(--cv-text-dim);font-size:.85rem;padding:16px;text-align:center}.cv-toast{position:fixed;bottom:20px;right:20px;padding:12px 20px;border-radius:var(--cv-radius);font-size:.9rem;animation:cv-slideIn .3s ease;z-index:1000}.cv-toast.success{background:var(--cv-success);color:#fff}.cv-toast.error{background:var(--cv-danger);color:#fff}@keyframes cv-slideIn{0%{transform:translate(100%);opacity:0}to{transform:translate(0);opacity:1}}@keyframes cv-pulse{0%,to{opacity:1}50%{opacity:.5}}@keyframes cv-spin{to{transform:rotate(360deg)}}.cv-editor-container{position:absolute;inset:0;display:flex;flex-direction:column;background:var(--cv-bg-alt);color:var(--cv-text);font-size:14px}.cv-editor-header{border-bottom:1px solid var(--cv-border);background:var(--cv-bg-alt)}.cv-editor-main{display:flex;flex:1;min-height:0;overflow:hidden;background:var(--cv-bg)}.cv-detail-title{font-size:.85rem;font-weight:500;color:var(--cv-text-muted);text-transform:uppercase;letter-spacing:.5px}.cv-detail-subtitle{font-size:.75rem;color:var(--cv-text-dim)}.cv-form-group{margin-bottom:16px}.cv-form-row{display:flex;gap:16px;margin-bottom:16px}.cv-form-row>.cv-form-group{flex:1;margin-bottom:0}.cv-form-row-3{display:grid;grid-template-columns:1fr 1fr 1fr;gap:8px;margin-bottom:8px}.cv-form-row-4{display:flex;gap:8px;margin-bottom:8px}.cv-form-row-4>.cv-form-group{flex:1;margin-bottom:0}.cv-label{display:block;font-size:.75rem;font-weight:500;color:var(--cv-text-dim);margin-bottom:6px;text-transform:uppercase;letter-spacing:.5px}.cv-checkbox-group{display:flex;align-items:center;gap:8px}.cv-checkbox{width:18px;height:18px;cursor:pointer;accent-color:var(--cv-accent)}.cv-form-group-sm{margin-bottom:8px}.cv-form-group-sm .cv-label{margin-bottom:2px}.cv-form-group-sm .cv-input,.cv-form-group-sm .cv-select{padding:4px 6px;font-size:12px}.cv-section{margin-bottom:24px;padding:16px;background:var(--cv-bg-alt);border:1px solid var(--cv-border);border-radius:var(--cv-radius)}.cv-section-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:12px}.cv-section-title{font-weight:600;font-size:14px}.cv-empty-state{display:flex;flex-direction:column;align-items:center;justify-content:center;height:100%;color:var(--cv-text-dim);text-align:center;padding:32px}.cv-empty-state-title{font-size:1rem;font-weight:500;margin-bottom:8px;color:var(--cv-text-muted)}.cv-empty-message{padding:24px;text-align:center;color:var(--cv-text-muted)}.cv-field{display:flex;gap:8px;font-size:13px;padding:4px 0}.cv-field-label{color:var(--cv-text-muted);white-space:nowrap}.cv-field-label:after{content:":"}.cv-field-value{color:var(--cv-text);font-family:var(--cv-font-mono)}.cv-field-value.text{font-family:inherit}.cv-msg-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:12px}.cv-msg-header-info{display:flex;align-items:center;gap:12px;flex-wrap:wrap}.cv-msg-title{font-weight:600;font-size:15px;color:#fff}.cv-msg-id{font-family:var(--cv-font-mono);color:var(--cv-text-muted);font-size:13px}.cv-msg-meta{font-size:12px;color:var(--cv-text-muted);padding:2px 8px;background:var(--cv-bg-elevated);border-radius:3px}.cv-msg-meta.dimmed{opacity:.6;font-style:italic;background:transparent}.cv-msg-actions{display:flex;gap:6px}.cv-bit-layout{background:var(--cv-bg);border:1px solid var(--cv-border);border-radius:var(--cv-radius);padding:8px 12px}.cv-bit-layout-header{display:flex;justify-content:space-between;align-items:center;margin-bottom:4px;font-size:10px;color:var(--cv-text-muted)}.cv-bit-bar{position:relative;height:10px;background:var(--cv-bg-elevated);border-radius:2px;overflow:hidden}.cv-bit-segment{position:absolute;top:0;height:100%;display:flex;align-items:center;justify-content:center;font-size:8px;font-family:var(--cv-font-mono);color:#ffffffe6;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;border-right:1px solid rgba(0,0,0,.3);box-sizing:border-box}.cv-bit-segment.current{z-index:2;box-shadow:0 0 0 1px #fff}.cv-bit-segment.overlap{background:repeating-linear-gradient(45deg,transparent,transparent 2px,rgba(255,0,0,.3) 2px,rgba(255,0,0,.3) 4px)!important}.cv-bit-markers{display:flex;justify-content:space-between;margin-top:2px;font-size:9px;font-family:var(--cv-font-mono);color:#444}.cv-bit-sliders{margin-top:8px;display:flex;gap:16px}.cv-bit-slider-group{flex:1}.cv-bit-slider-label{display:flex;justify-content:space-between;font-size:10px;color:var(--cv-text-muted);margin-bottom:2px}.cv-bit-slider-value{font-family:var(--cv-font-mono);color:var(--cv-text)}.cv-bit-slider{width:100%;height:6px;-webkit-appearance:none;appearance:none;background:#333;border-radius:3px;outline:none;cursor:pointer}.cv-bit-slider::-webkit-slider-thumb{-webkit-appearance:none;width:14px;height:14px;background:var(--cv-accent);border-radius:50%;cursor:pointer;border:2px solid #fff}.cv-bit-slider::-moz-range-thumb{width:14px;height:14px;background:var(--cv-accent);border-radius:50%;cursor:pointer;border:2px solid #fff}.cv-signals-section{flex:1;display:flex;flex-direction:column;min-height:0;max-height:500px}.cv-signals-layout{display:flex;gap:12px;flex:1;min-height:0;min-width:0;overflow:hidden;padding:8px}.cv-signals-table-container{flex:1;min-width:0;overflow:auto;border:1px solid var(--cv-border);border-radius:var(--cv-radius);display:flex;flex-direction:column}.cv-signals-table-container cv-signals-table{padding:4px 8px}.cv-signals-table-header{display:flex;justify-content:space-between;align-items:center;padding:6px 10px;background:var(--cv-bg-elevated);border-bottom:1px solid var(--cv-border);font-size:.8rem;font-weight:500;color:var(--cv-text-muted);flex-shrink:0}.cv-signal-editor-panel{width:340px;flex-shrink:0;padding:12px;background:var(--cv-bg-alt);border:1px solid var(--cv-border);border-radius:var(--cv-radius);max-height:520px;overflow-y:auto}.cv-nodes-list{display:flex;flex-wrap:wrap;gap:8px;margin-bottom:12px}.cv-node-tag{display:inline-flex;align-items:center;gap:6px;padding:6px 10px;background:var(--cv-bg-elevated);border:1px solid var(--cv-border);border-radius:var(--cv-radius);font-size:13px}.cv-node-remove{display:inline-flex;align-items:center;justify-content:center;width:18px;height:18px;border:none;border-radius:50%;background:transparent;color:var(--cv-text-muted);font-size:14px;cursor:pointer;transition:all .15s ease}.cv-node-remove:hover{background:var(--cv-danger);color:#fff}.cv-add-node-form{display:flex;gap:8px}.cv-add-node-form .cv-input{flex:1}.cv-preview-panel{flex:1;margin:16px}.cv-preview-content{flex:1;overflow:auto;padding:0;background:var(--cv-bg)}.cv-preview-text{margin:0;padding:16px;font-family:var(--cv-font-mono);font-size:12px;line-height:1.5;color:var(--cv-text);white-space:pre;overflow-x:auto}.cv-help-text{color:var(--cv-text-dim);font-size:.85rem;margin-bottom:16px;line-height:1.5}.cv-actions{display:flex;gap:8px;margin-top:16px}.cv-btn.warning{background:#f59e0b;color:#fff;border-color:#f59e0b}.cv-btn.warning:hover:not(:disabled){background:#d97706;border-color:#d97706}.cv-error-msg{color:var(--cv-danger);font-size:11px;margin-top:4px}.cv-id-display{font-family:var(--cv-font-mono);color:var(--cv-text-muted);font-size:12px;margin-left:8px}.cv-list-item-compact{padding:6px 12px}.cv-modal-overlay{position:fixed;inset:0;background:#0009;display:flex;align-items:center;justify-content:center;z-index:1000}.cv-modal{background:var(--cv-bg-elevated);border:1px solid var(--cv-border);border-radius:var(--cv-radius);min-width:360px;max-width:480px;box-shadow:0 8px 32px #0006}.cv-modal-header{padding:12px 16px;font-weight:600;font-size:.95rem;color:var(--cv-text);border-bottom:1px solid var(--cv-border)}.cv-modal-body{padding:16px;color:var(--cv-text-muted);font-size:.9rem}.cv-modal-body p{margin:0 0 8px}.cv-modal-body p:last-child{margin-bottom:0}.cv-modal-filename{font-family:var(--cv-font-mono);font-size:.85rem;color:var(--cv-accent);background:var(--cv-bg);padding:8px 12px;border-radius:var(--cv-radius);word-break:break-all}.cv-modal-actions{display:flex;gap:8px;padding:12px 16px;border-top:1px solid var(--cv-border);justify-content:flex-end}.cv-markdown-content{padding:24px;color:var(--cv-text);line-height:1.6;overflow-y:auto;max-height:calc(100vh - 200px)}.cv-markdown-content h1{font-size:1.5rem;font-weight:600;color:var(--cv-text);margin:0 0 16px;padding-bottom:8px;border-bottom:1px solid var(--cv-border)}.cv-markdown-content h2{font-size:1.25rem;font-weight:600;color:var(--cv-text);margin:24px 0 12px;padding-bottom:6px;border-bottom:1px solid var(--cv-border)}.cv-markdown-content h3{font-size:1.1rem;font-weight:600;color:var(--cv-text);margin:20px 0 10px}.cv-markdown-content h4{font-size:1rem;font-weight:600;color:var(--cv-text-muted);margin:16px 0 8px}.cv-markdown-content p{margin:0 0 12px}.cv-markdown-content ul,.cv-markdown-content ol{margin:0 0 12px;padding-left:24px}.cv-markdown-content li{margin:4px 0}.cv-markdown-content strong{font-weight:600;color:var(--cv-text)}.cv-markdown-content code{font-family:var(--cv-font-mono);font-size:.9em;background:var(--cv-bg);padding:2px 6px;border-radius:3px;color:var(--cv-accent)}.cv-markdown-content pre{background:var(--cv-bg);border:1px solid var(--cv-border);border-radius:var(--cv-radius);padding:12px 16px;overflow-x:auto;margin:0 0 16px}.cv-markdown-content pre code{background:none;padding:0;color:var(--cv-text);font-size:.85rem}.cv-markdown-content table{width:100%;border-collapse:collapse;margin:0 0 16px;font-size:.9rem}.cv-markdown-content th,.cv-markdown-content td{border:1px solid var(--cv-border);padding:8px 12px;text-align:left}.cv-markdown-content th{background:var(--cv-bg);font-weight:600;color:var(--cv-text)}.cv-markdown-content tr:nth-child(2n){background:#ffffff05}.cv-markdown-content hr{border:none;border-top:1px solid var(--cv-border);margin:24px 0}.cv-markdown-content blockquote{border-left:3px solid var(--cv-accent);margin:0 0 16px;padding:8px 16px;background:#3b82f60d;color:var(--cv-text-muted)}.cv-markdown-content a{color:var(--cv-accent);text-decoration:none}.cv-markdown-content a:hover{text-decoration:underline}',Tt=500;class Dt extends HTMLElement{frames=[];selectedIndex=null;messageInfoLookup=()=>({name:"-"});delegatedHandler=null;constructor(){super()}connectedCallback(){this.setupEventDelegation()}disconnectedCallback(){this.removeEventDelegation()}setupEventDelegation(){const e=this.querySelector("tbody");!e||this.delegatedHandler||(this.delegatedHandler=t=>{const a=t.target.closest("tr.clickable");a?.dataset.index&&this.selectFrame(parseInt(a.dataset.index,10))},e.addEventListener("click",this.delegatedHandler))}removeEventDelegation(){if(!this.delegatedHandler)return;const e=this.querySelector("tbody");e&&e.removeEventListener("click",this.delegatedHandler),this.delegatedHandler=null}setMessageNameLookup(e){this.messageInfoLookup=t=>({name:e(t)})}setMessageInfoLookup(e){this.messageInfoLookup=e}setFrames(e){this.frames=e,this.render()}get frameCount(){return this.frames.length}clearSelection(){this.selectedIndex=null,this.updateSelection()}render(){const e=this.querySelector("tbody");if(!e)return;const t=Math.max(0,this.frames.length-Tt),s=this.frames.slice(t);e.innerHTML=s.map((a,n)=>{const r=t+n,o=this.messageInfoLookup(a.can_id),l=o.name!=="-",p=["clickable",r===this.selectedIndex?"selected":"",l?"matched":""].filter(Boolean).join(" "),c=o.comment?g(o.comment):"";return`
      <tr class="${p}" data-index="${r}">
        <td class="cv-cell-dim">${rt(a.timestamp)}</td>
        <td>${a.channel}</td>
        <td class="cv-cell-id" title="${a.can_id}">${P(a.can_id,a.is_extended)}</td>
        <td class="${l?"cv-cell-name":"cv-cell-nomatch"}"${c?` title="${c}"`:""}>${o.name}</td>
        <td>${a.dlc}</td>
        <td class="cv-cell-data">${it(a.data)}</td>
        <td>${nt(a)}</td>
      </tr>
    `}).join(""),this.delegatedHandler||this.setupEventDelegation()}selectFrame(e){this.selectedIndex=e,this.updateSelection();const t=this.frames[e];t&&this.dispatchEvent(new CustomEvent("frame-selected",{detail:{frame:t,index:e},bubbles:!0}))}updateSelection(){const e=this.querySelector("tbody");e&&e.querySelectorAll("tr").forEach(t=>{const s=parseInt(t.dataset.index||"-1",10);t.classList.toggle("selected",s===this.selectedIndex)})}}customElements.define("cv-frames-table",Dt);class Ft extends HTMLElement{signals=[];errors=[];api=null;currentFrame=null;handleFrameSelected=e=>this.onFrameSelected(e);handleDbcChanged=e=>this.onDbcChanged(e);constructor(){super()}connectedCallback(){f.on("frame:selected",this.handleFrameSelected),f.on("dbc:changed",this.handleDbcChanged)}disconnectedCallback(){f.off("frame:selected",this.handleFrameSelected),f.off("dbc:changed",this.handleDbcChanged)}setApi(e){this.api=e}async onFrameSelected(e){if(this.currentFrame=e.frame,e.signals.length>0){this.setSignals(e.signals,[]);return}await this.decodeCurrentFrame()}async onDbcChanged(e){if(e.action==="cleared"){this.currentFrame&&this.setSignals([],[]);return}this.currentFrame&&await this.decodeCurrentFrame()}async decodeCurrentFrame(){const e=this.currentFrame;if(!e||!this.api){this.setSignals([],[]);return}try{const t=await this.api.decodeFrames([e]);this.currentFrame===e&&this.setSignals(t.signals,t.errors)}catch(t){console.error("Failed to decode frame:",t),this.currentFrame===e&&this.setSignals([],[])}}setSignals(e,t=[]){this.signals=e,this.errors=t,this.render()}showEmpty(){this.signals=[],this.errors=[];const e=this.querySelector("tbody"),t=this.querySelector("#signalsCount"),s=this.querySelector(".cv-decode-error");e&&(e.innerHTML='<tr><td colspan="3" class="cv-signals-empty">Select a frame to view decoded signals</td></tr>'),t&&(t.textContent="Select a frame"),s&&(s.textContent="",s.classList.add("hidden"))}clear(){this.currentFrame=null,this.showEmpty()}render(){const e=this.querySelector("tbody"),t=this.querySelector("#signalsCount"),s=this.querySelector(".cv-decode-error");e&&(this.signals.length===0&&this.errors.length===0?e.innerHTML='<tr><td colspan="3" class="cv-signals-empty">No signals decoded</td></tr>':e.innerHTML=this.signals.map(a=>{const n=a.description?`<span class="cv-value-desc">${g(a.description)}</span> <span class="cv-value-num">(${fe(a.value)})</span>`:fe(a.value);return`
            <tr>
              <td class="cv-signal-name">${g(a.signal_name)}</td>
              <td class="cv-physical-value">${n}</td>
              <td class="cv-unit-highlight">${a.unit||"-"}</td>
            </tr>
          `}).join("")),t&&(t.textContent=`${this.signals.length} signals`),s&&(this.errors.length>0?(s.textContent=this.errors.join("; "),s.classList.remove("hidden")):(s.textContent="",s.classList.add("hidden")))}}customElements.define("cv-signals-panel",Ft);class At extends HTMLElement{constructor(){super()}connectedCallback(){this.bindEvents()}bindEvents(){this.querySelectorAll(".cv-input").forEach(a=>{a.addEventListener("input",()=>{this.updateInputHighlight(a),this.emitFilterChange()}),a.addEventListener("keydown",n=>{n.key==="Enter"&&this.flashConfirm(a)}),this.updateInputHighlight(a)}),this.querySelectorAll(".cv-select").forEach(a=>{a.addEventListener("change",()=>{this.updateSelectHighlight(a),this.emitFilterChange()}),this.updateSelectHighlight(a)}),this.querySelector("#clearFiltersBtn")?.addEventListener("click",()=>this.clearFilters())}updateInputHighlight(e){e.classList.toggle("filter-active",e.value.trim()!=="")}updateSelectHighlight(e){e.classList.toggle("filter-active",e.value!=="all")}flashConfirm(e){e.classList.add("filter-confirm"),setTimeout(()=>e.classList.remove("filter-confirm"),300)}getFilters(){const e=this.getInputValue("filterTimeMin"),t=this.getInputValue("filterTimeMax"),s=this.getInputValue("filterCanId"),a=this.getInputValue("filterMessage"),n=this.getInputValue("filterSignal"),r=this.getInputValue("filterDataPattern"),o=this.getInputValue("filterChannel"),l=this.getSelectValue("filterMatchStatus");return{timeMin:e?parseFloat(e):null,timeMax:t?parseFloat(t):null,canIds:yt(s),messages:ye(a),signals:ye(n),dataPattern:ut(r),channel:o||null,matchStatus:l||"all"}}clearFilters(){this.setInputValue("filterTimeMin",""),this.setInputValue("filterTimeMax",""),this.setInputValue("filterCanId",""),this.setInputValue("filterMessage",""),this.setInputValue("filterSignal",""),this.setInputValue("filterDataPattern",""),this.setInputValue("filterChannel",""),this.setSelectValue("filterMatchStatus","all"),this.querySelectorAll(".filter-active").forEach(e=>e.classList.remove("filter-active")),this.emitFilterChange()}updateSummary(e,t){const s=this.querySelector("#filterSummary");if(s){const a=this.getFilters(),n=ze(a);n===0?s.textContent="No filters active":s.textContent=`${n} filter${n>1?"s":""} · ${e}/${t} frames`}}getInputValue(e){return this.querySelector(`#${e}`)?.value.trim()||""}setInputValue(e,t){const s=this.querySelector(`#${e}`);s&&(s.value=t)}getSelectValue(e){return this.querySelector(`#${e}`)?.value||""}setSelectValue(e,t){const s=this.querySelector(`#${e}`);s&&(s.value=t)}emitFilterChange(){this.dispatchEvent(new CustomEvent("filter-change",{detail:this.getFilters(),bubbles:!0}))}}customElements.define("cv-filters-panel",At);function Rt(){return{frames:[],filteredFrames:[],filters:xt(),selectedFrameIndex:null,dbcInfo:null,currentFile:null}}class Bt extends HTMLElement{api=null;state;shadow;framesTable=null;signalsPanel=null;filtersPanel=null;handleDbcChanged=e=>this.onDbcChanged(e);handleCaptureStopped=e=>this.onCaptureStopped(e);unsubscribeAppStore=null;constructor(){super(),this.state=Rt(),this.shadow=this.attachShadow({mode:"open"})}connectedCallback(){this.render(),f.on("dbc:changed",this.handleDbcChanged),f.on("capture:stopped",this.handleCaptureStopped),this.unsubscribeAppStore=k.subscribe(e=>this.onAppStoreChange(e.mdf4File))}disconnectedCallback(){f.off("dbc:changed",this.handleDbcChanged),f.off("capture:stopped",this.handleCaptureStopped),this.unsubscribeAppStore?.()}onAppStoreChange(e){e&&e!==this.state.currentFile?this.loadFile(e):!e&&this.state.currentFile&&this.clearAllData()}onCaptureStopped(e){const t=e.captureFile;t&&t===this.state.currentFile&&this.loadFile(t)}setApi(e){this.api=e,this.refreshDbcInfo()}getFrames(){return this.state.frames}async onDbcChanged(e){e.dbcInfo?this.state.dbcInfo=e.dbcInfo:e.action!=="cleared"?await this.refreshDbcInfo():this.state.dbcInfo=null,this.state.currentFile&&e.action!=="cleared"?await this.reloadCurrentFile():this.renderFrames()}async reloadCurrentFile(){if(!(!this.api||!this.state.currentFile))try{const[e,t]=await this.api.loadMdf4(this.state.currentFile);this.state.frames=e,this.state.filteredFrames=[...e],k.set({mdf4Frames:e,mdf4Signals:t}),this.renderFrames(),this.signalsPanel?.clear()}catch(e){console.error("Failed to reload MDF4:",e)}}async refreshDbcInfo(){if(this.api)try{this.state.dbcInfo=await this.api.getDbcInfo(),this.renderFrames()}catch{}}async loadFile(e){if(!this.api)return;const t=this.shadow.querySelector("#loadBtn");try{t&&(t.disabled=!0,t.textContent="Loading...");const[s,a]=await this.api.loadMdf4(e);this.state.frames=s,this.state.filteredFrames=[...s],this.state.selectedFrameIndex=null,this.state.currentFile=e,k.set({mdf4File:e,mdf4Frames:s,mdf4Signals:a}),this.renderFrames(),this.signalsPanel?.clear(),this.showMessage(`Loaded ${s.length} frames`),xe({action:"loaded"})}catch(s){this.showMessage(String(s),"error")}finally{t&&(t.disabled=!1,t.textContent="Open")}}render(){this.shadow.innerHTML=`
      <style>${L}</style>
      ${this.generateTemplate()}
    `,this.cacheElements(),this.bindEvents()}generateTemplate(){return`
      <div class="cv-mdf4-inspector">
        <div class="cv-panel">
          <div class="cv-panel-header">
            <div class="cv-tabs">
              <button class="cv-tab active" data-tab="data">CAN Frames <span class="cv-tab-badge" id="framesCount">0</span></button>
              <button class="cv-tab" data-tab="filters">Filters <span class="cv-tab-badge dimmed" id="filterCount">0</span></button>
            </div>
          </div>
          <div class="cv-panel-body flush">
            ${this.generateDataSection()}
            ${this.generateFiltersSection()}
          </div>
        </div>
      </div>
    `}generateDataSection(){return`
      <div class="cv-tab-pane active" id="dataSection">
        <div class="cv-grid responsive">
          <cv-frames-table class="cv-card" id="framesTable">
            <div class="cv-card-header">
              <span class="cv-card-title">Raw CAN Frames</span>
            </div>
            <div class="cv-filter-chips" id="filterChips"></div>
            <div class="cv-table-wrap">
              <table class="cv-table">
                <thead>
                  <tr>
                    <th>Timestamp</th>
                    <th>Channel</th>
                    <th>CAN ID</th>
                    <th>Message</th>
                    <th>DLC</th>
                    <th>Data</th>
                    <th>Flags</th>
                  </tr>
                </thead>
                <tbody id="framesTableBody"></tbody>
              </table>
            </div>
          </cv-frames-table>
          <cv-signals-panel class="cv-card" id="signalsPanel">
            <div class="cv-card-header">
              <span class="cv-card-title">Decoded Signals <span class="cv-tab-badge" id="signalsCount">0</span></span>
              <span class="cv-decode-error hidden"></span>
            </div>
            <div class="cv-table-wrap">
              <table class="cv-table">
                <thead>
                  <tr>
                    <th>Signal</th>
                    <th>Value</th>
                    <th>Unit</th>
                  </tr>
                </thead>
                <tbody id="signalsTableBody"></tbody>
              </table>
            </div>
          </cv-signals-panel>
        </div>
      </div>
    `}generateFiltersSection(){return`
      <cv-filters-panel class="cv-tab-pane" id="filtersSection">
        <div class="cv-filters-grid">
          <div class="cv-filter-section">
            <div class="cv-filter-section-title">Frame Filters</div>
            <div class="cv-filter-row">
              <label>Time Range:</label>
              <input type="text" class="cv-input mono" id="filterTimeMin" placeholder="0.000">
              <span class="cv-filter-sep">to</span>
              <input type="text" class="cv-input mono" id="filterTimeMax" placeholder="999.999">
            </div>
            <div class="cv-filter-row">
              <label>CAN ID:</label>
              <input type="text" class="cv-input mono" id="filterCanId" placeholder="7DF, 7E8, 100-1FF">
            </div>
            <div class="cv-filter-row">
              <label>Channel:</label>
              <input type="text" class="cv-input mono" id="filterChannel" placeholder="can0, vcan0">
            </div>
            <div class="cv-filter-row">
              <label>Data Pattern:</label>
              <input type="text" class="cv-input mono" id="filterDataPattern" placeholder="01 ?? FF (use ?? for wildcard)">
            </div>
          </div>
          <div class="cv-filter-section">
            <div class="cv-filter-section-title">DBC Filters</div>
            <div class="cv-filter-row">
              <label>Message:</label>
              <input type="text" class="cv-input mono" id="filterMessage" placeholder="EngineData, VehicleSpeed">
            </div>
            <div class="cv-filter-row">
              <label>Signal:</label>
              <input type="text" class="cv-input mono" id="filterSignal" placeholder="RPM, Temperature">
            </div>
            <div class="cv-filter-row">
              <label>Match Status:</label>
              <select class="cv-select" id="filterMatchStatus">
                <option value="all">All Frames</option>
                <option value="matched">Matched Only</option>
                <option value="unmatched">Unmatched Only</option>
              </select>
            </div>
          </div>
          <div class="cv-filter-section cv-filter-actions">
            <button class="cv-btn" id="clearFiltersBtn">Clear All Filters</button>
            <div class="cv-filter-summary">
              <span id="filterSummary">No filters active</span>
            </div>
          </div>
        </div>
      </cv-filters-panel>
    `}cacheElements(){this.framesTable=this.shadow.querySelector("cv-frames-table"),this.signalsPanel=this.shadow.querySelector("cv-signals-panel"),this.filtersPanel=this.shadow.querySelector("cv-filters-panel"),this.framesTable?.setMessageInfoLookup(e=>this.getMessageInfo(e)),this.signalsPanel&&this.api&&this.signalsPanel.setApi({decodeFrames:e=>this.api.decodeFrames(e)})}bindEvents(){this.shadow.querySelector("#loadBtn")?.addEventListener("click",()=>this.promptLoadMdf4()),this.shadow.querySelector("#clearBtn")?.addEventListener("click",()=>this.clearAllData()),this.shadow.querySelectorAll(".cv-tabs .cv-tab").forEach(e=>{e.addEventListener("click",()=>{const t=e.dataset.tab;t&&this.switchTab(t)})}),this.framesTable?.addEventListener("frame-selected",e=>{const t=e.detail;this.state.selectedFrameIndex=t.index;const s=k.get().mdf4Signals,a=t.frame.can_id,r=this.state.dbcInfo?.messages.find(l=>l.id===a)?.name,o=s.filter(l=>l.timestamp===t.frame.timestamp&&(!r||l.message_name===r));ct({frame:t.frame,index:t.index,source:"mdf4-inspector",signals:o})}),this.filtersPanel?.addEventListener("filter-change",e=>{const t=e.detail;this.state.filters=t,this.applyFilters(),this.renderFrames(),this.signalsPanel?.clear(),this.updateFilterChips()})}switchTab(e){this.shadow.querySelectorAll(".cv-tabs .cv-tab").forEach(t=>t.classList.toggle("active",t.dataset.tab===e)),this.shadow.querySelectorAll(".cv-panel-body > .cv-tab-pane").forEach(t=>t.classList.toggle("active",t.id===`${e}Section`))}async promptLoadMdf4(){if(this.api)try{const e=await this.api.openFileDialog([{name:"MDF4 Files",extensions:["mf4","mdf","mdf4","MF4","MDF","MDF4"]}]);e&&await this.loadFile(e)}catch(e){this.showMessage(String(e),"error")}}clearAllData(){this.state.frames=[],this.state.filteredFrames=[],this.state.selectedFrameIndex=null,this.state.currentFile=null,k.set({mdf4File:null,mdf4Frames:[],mdf4Signals:[]}),this.renderFrames(),this.signalsPanel?.clear(),xe({action:"cleared"})}applyFilters(){this.state.filteredFrames=wt(this.state.frames,this.state.filters,this.state.dbcInfo)}renderFrames(){this.applyFilters(),this.framesTable?.setFrames(this.state.filteredFrames),this.updateFrameCount(),this.updateFilterTabBadge()}getMessageInfo(e){if(!this.state.dbcInfo)return{name:"-"};const t=this.state.dbcInfo.messages.find(s=>s.id===e);return t?{name:t.name,comment:t.comment}:{name:"-"}}updateFrameCount(){const e=this.shadow.querySelector("#framesCount");e&&(e.textContent=String(this.state.filteredFrames.length))}updateFilterTabBadge(){const e=this.shadow.querySelector("#filterCount");if(e){const t=ze(this.state.filters);e.textContent=String(t),e.classList.toggle("active",t>0)}}updateFilterChips(){const e=this.shadow.querySelector("#filterChips");if(!e)return;const t=this.state.filters,s=[];if(t.timeMin!==null&&s.push({label:`Time ≥ ${t.timeMin}`,key:"timeMin",value:""}),t.timeMax!==null&&s.push({label:`Time ≤ ${t.timeMax}`,key:"timeMax",value:""}),t.canIds?.length&&s.push({label:`ID: ${t.canIds.map(a=>"0x"+a.toString(16).toUpperCase()).join(", ")}`,key:"canIds",value:""}),t.channel&&s.push({label:`Ch: ${t.channel}`,key:"channel",value:""}),t.dataPattern&&s.push({label:`Data: ${t.dataPattern}`,key:"dataPattern",value:""}),t.messages?.length&&s.push({label:`Msg: ${t.messages.join(", ")}`,key:"messages",value:""}),t.signals?.length&&s.push({label:`Sig: ${t.signals.join(", ")}`,key:"signals",value:""}),t.matchStatus!=="all"&&s.push({label:t.matchStatus==="matched"?"Matched only":"Unmatched only",key:"matchStatus",value:"all"}),s.length===0){e.innerHTML="";return}e.innerHTML=s.map(a=>`<span class="cv-filter-chip" data-key="${a.key}" data-value="${a.value}">${a.label}<button class="cv-filter-chip-x" title="Remove filter">&times;</button></span>`).join(""),e.querySelectorAll(".cv-filter-chip-x").forEach(a=>{a.addEventListener("click",n=>{n.stopPropagation();const r=n.target.closest(".cv-filter-chip");r&&this.clearFilter(r.dataset.key,r.dataset.value)})})}clearFilter(e,t){const a={timeMin:"filterTimeMin",timeMax:"filterTimeMax",canIds:"filterCanId",channel:"filterChannel",dataPattern:"filterDataPattern",messages:"filterMessage",signals:"filterSignal",matchStatus:"filterMatchStatus"}[e];if(!a)return;const n=this.filtersPanel?.querySelector(`#${a}`);n&&(n.value=t,n.classList.remove("filter-active"),this.filtersPanel?.dispatchEvent(new CustomEvent("filter-change",{detail:this.filtersPanel?.getFilters?.()??this.state.filters,bubbles:!0})))}showMessage(e,t="success"){const s=document.createElement("div");s.className=`cv-message ${t}`,s.textContent=e,this.shadow.appendChild(s),setTimeout(()=>s.remove(),3e3)}}customElements.define("cv-mdf4-inspector",Bt);class qt extends HTMLElement{api=null;state;shadow;latestUpdate=null;unlisteners=[];unsubscribeAppStore=null;handleDbcChanged=e=>this.onDbcChanged(e);constructor(){super(),this.state={isCapturing:!1,currentInterface:null,captureFile:null},this.shadow=this.attachShadow({mode:"open"})}connectedCallback(){this.render(),f.on("dbc:changed",this.handleDbcChanged),this.unsubscribeAppStore=k.subscribe(e=>this.onAppStoreChange(e.mdf4File))}disconnectedCallback(){this.cleanup(),f.off("dbc:changed",this.handleDbcChanged),this.unsubscribeAppStore?.()}setApi(e){this.api=e,this.setupEventListeners(),this.loadInterfaces()}onDbcChanged(e){this.latestUpdate&&this.renderFromUpdate(this.latestUpdate)}onAppStoreChange(e){e!==this.state.captureFile&&(this.state.captureFile=e)}setupEventListeners(){this.api&&this.unlisteners.push(this.api.onLiveCaptureUpdate(e=>this.handleUpdate(e)),this.api.onCaptureFinalized(e=>this.handleFinalized(e)),this.api.onCaptureError(e=>{console.error("[live-viewer] CAPTURE ERROR:",e),this.showMessage(e,"error"),this.state.isCapturing=!1,this.updateStoreStatus()}))}cleanup(){this.unlisteners.forEach(e=>e()),this.unlisteners=[]}handleUpdate(e){this.state.isCapturing&&(this.latestUpdate=e,this.renderFromUpdate(e),this.updateStoreStatus())}handleFinalized(e){this.state.captureFile=e,this.showMessage(`MDF4 saved to ${$(e)}`)}render(){this.shadow.innerHTML=`
      <style>${L}</style>
      ${this.generateTemplate()}
    `,this.bindEvents()}generateTemplate(){return`
      <div class="cv-live-viewer">
        <div class="cv-panel">
          <div class="cv-panel-header">
            <div class="cv-tabs">
              <button class="cv-tab active" data-tab="monitor">Message Monitor <span class="cv-tab-badge" id="messageCount">0</span></button>
              <button class="cv-tab" data-tab="signals">Signal Monitor <span class="cv-tab-badge" id="signalCount">0</span></button>
              <button class="cv-tab" data-tab="stream">Frame Stream <span class="cv-tab-badge" id="frameCount">0</span></button>
              <button class="cv-tab" data-tab="errors">Error Monitor <span class="cv-tab-badge cv-tab-badge-error dimmed" id="errorCount" data-count="0">0</span></button>
            </div>
          </div>
          <div class="cv-panel-body flush">
            ${this.generateMonitorSection()}
            ${this.generateSignalsSection()}
            ${this.generateStreamSection()}
            ${this.generateErrorsSection()}
          </div>
        </div>

        <div class="cv-stats-bar">
          <div class="cv-stat">
            <span class="cv-stat-label">Messages</span>
            <span class="cv-stat-value" id="statMsgCount">0</span>
          </div>
          <div class="cv-stat">
            <span class="cv-stat-label">Frames</span>
            <span class="cv-stat-value" id="statTotalFrames">0</span>
          </div>
          <div class="cv-stat">
            <span class="cv-stat-label">Rate</span>
            <span class="cv-stat-value" id="statFrameRate">0/s</span>
          </div>
          <div class="cv-stat">
            <span class="cv-stat-label">Elapsed</span>
            <span class="cv-stat-value" id="statElapsed">0:00</span>
          </div>
        </div>
      </div>
    `}generateMonitorSection(){return`
      <div class="cv-tab-pane active" id="monitorSection">
        <div class="cv-table-wrap">
          <table class="cv-table">
            <thead>
              <tr>
                <th>CAN ID</th>
                <th>Message</th>
                <th>Data</th>
                <th>Count</th>
                <th>Rate</th>
              </tr>
            </thead>
            <tbody id="monitorTableBody"></tbody>
          </table>
        </div>
      </div>
    `}generateSignalsSection(){return`
      <div class="cv-tab-pane" id="signalsSection">
        <div class="cv-signal-monitor" id="signalsContainer"></div>
      </div>
    `}generateStreamSection(){return`
      <div class="cv-tab-pane" id="streamSection">
        <div class="cv-table-wrap">
          <table class="cv-table">
            <thead>
              <tr>
                <th>Timestamp</th>
                <th>CAN ID</th>
                <th>DLC</th>
                <th>Data</th>
                <th>Flags</th>
              </tr>
            </thead>
            <tbody id="streamTableBody"></tbody>
          </table>
        </div>
      </div>
    `}generateErrorsSection(){return`
      <div class="cv-tab-pane" id="errorsSection">
        <div class="cv-table-wrap">
          <table class="cv-table">
            <thead>
              <tr>
                <th>Timestamp</th>
                <th>Channel</th>
                <th>Error Type</th>
                <th>Details</th>
                <th>Count</th>
              </tr>
            </thead>
            <tbody id="errorsTableBody"></tbody>
          </table>
        </div>
      </div>
    `}bindEvents(){this.shadow.querySelectorAll(".cv-tabs .cv-tab").forEach(e=>{e.addEventListener("click",()=>{const t=e.dataset.tab;t&&this.switchTab(t)})})}switchTab(e){this.shadow.querySelectorAll(".cv-tabs .cv-tab").forEach(t=>t.classList.toggle("active",t.dataset.tab===e)),this.shadow.querySelectorAll(".cv-panel-body > .cv-tab-pane").forEach(t=>t.classList.toggle("active",t.id===`${e}Section`))}async loadInterfaces(){if(this.api)try{const e=await this.api.listCanInterfaces();pt({interfaces:e})}catch(e){console.warn("Could not load interfaces:",e)}}async startCapture(e){if(this.api)try{const t=k.get().mdf4File;let s=null,a=!1;if(t){const l=await this.showCaptureChoiceDialog(t);if(l==="cancel")return;l==="append"?(s=t,a=!0):l==="overwrite"&&(s=t,a=!1)}if(!s){const p=`can-capture-${new Date().toISOString().replace(/[:.]/g,"-").slice(0,19)}.mf4`;if(s=await this.api.saveFileDialog([{name:"MDF4 Files",extensions:["mf4"]}],p),!s)return}a||(this.latestUpdate=null,this.renderFromUpdate(null)),this.state.isCapturing=!0,this.state.currentInterface=e,this.state.captureFile=s,this.updateStoreStatus();const n=this.api.getFiltersForInterface?.(e);await this.api.startCapture(e,s,a,n);const r=a?"Appending to":"Capturing to",o=n?.length?` (${n.length} filter${n.length>1?"s":""})`:"";this.showMessage(`${r} ${$(s)}${o}`),dt({interface:e,captureFile:s})}catch(t){console.error("[live-viewer] startCapture FAILED:",t),this.state.isCapturing=!1,this.updateStoreStatus(),this.showMessage(String(t),"error")}}showCaptureChoiceDialog(e){return new Promise(t=>{const s=$(e),a=document.createElement("div");a.className="cv-modal-overlay",a.innerHTML=`
        <div class="cv-modal">
          <div class="cv-modal-header">MDF4 File Already Selected</div>
          <div class="cv-modal-body">
            <p>An MDF4 file is currently selected:</p>
            <p class="cv-modal-filename">${s}</p>
            <p>What would you like to do?</p>
          </div>
          <div class="cv-modal-actions">
            <button class="cv-btn success" data-action="append">Append</button>
            <button class="cv-btn warning" data-action="overwrite">Overwrite</button>
            <button class="cv-btn primary" data-action="new">New File</button>
            <button class="cv-btn" data-action="cancel">Cancel</button>
          </div>
        </div>
      `;const n=r=>{const l=r.target.dataset.action;l&&(a.remove(),t(l))};a.addEventListener("click",n),this.shadow.appendChild(a)})}async stopCapture(){if(this.api)try{const e=await this.api.stopCapture(),t=this.latestUpdate?.stats.frame_count??0,s=this.state.currentInterface;this.state.isCapturing=!1,this.state.captureFile=e,this.updateStoreStatus(),this.showMessage(`Capture saved to ${$(e)}`),ht({interface:s,captureFile:e,frameCount:t})}catch(e){this.state.isCapturing=!1,this.updateStoreStatus(),this.showMessage(String(e),"error")}}clearAllData(){this.latestUpdate=null,this.renderFromUpdate(null),this.updateStoreStatus()}updateStoreStatus(){const e={isCapturing:this.state.isCapturing,currentInterface:this.state.currentInterface,frameCount:this.latestUpdate?.stats.frame_count??0,messageCount:this.latestUpdate?.stats.message_count??0};B.set(e),this.state.isCapturing||k.set({mdf4File:this.state.captureFile})}getIsCapturing(){return this.state.isCapturing}getFrameCount(){return this.latestUpdate?.stats.frame_count??0}getCaptureFile(){return this.state.captureFile}renderFromUpdate(e){const t=this.shadow.querySelector("#monitorTableBody");t&&(t.innerHTML=e?.messages_html??"");const s=this.shadow.querySelector("#signalsContainer");s&&(s.innerHTML=e?.signals_html??"");const a=this.shadow.querySelector("#streamTableBody");a&&(a.innerHTML=e?.frames_html??"");const n=this.shadow.querySelector("#errorsTableBody");n&&(n.innerHTML=e?.errors_html??"");const r=this.shadow.querySelector("#messageCount");r&&(r.textContent=String(e?.message_count??0));const o=this.shadow.querySelector("#signalCount");o&&(o.textContent=String(e?.signal_count??0));const l=this.shadow.querySelector("#frameCount");l&&(l.textContent=String(e?.frame_count??0));const p=this.shadow.querySelector("#errorCount");if(p){const v=e?.error_count??0;p.textContent=String(v),p.setAttribute("data-count",String(v)),p.classList.toggle("dimmed",v===0)}const c=this.shadow.querySelector("#statMsgCount"),d=this.shadow.querySelector("#statTotalFrames"),h=this.shadow.querySelector("#statFrameRate"),u=this.shadow.querySelector("#statElapsed");e?.stats_html?(c&&(c.textContent=e.stats_html.message_count),d&&(d.textContent=e.stats_html.frame_count),h&&(h.textContent=e.stats_html.frame_rate),u&&(u.textContent=e.stats_html.elapsed)):(c&&(c.textContent="0"),d&&(d.textContent="0"),h&&(h.textContent="0/s"),u&&(u.textContent="0:00"))}showMessage(e,t="success"){const s=document.createElement("div");s.className=`cv-message ${t}`,s.textContent=e,this.shadow.appendChild(s),setTimeout(()=>s.remove(),3e3)}}customElements.define("cv-live-viewer",qt);function K(){return{name:"",start_bit:0,length:8,byte_order:"little_endian",is_unsigned:!0,factor:1,offset:0,min:0,max:255,unit:null,receivers:{type:"none"},is_multiplexer:!1,multiplexer_value:null,comment:null}}function Q(i=8){return{id:0,is_extended:!1,name:"",dlc:i,sender:"Vector__XXX",signals:[],comment:null}}function $e(){return{version:null,bit_timing:null,nodes:[],messages:[],comment:null,value_descriptions:[],attribute_definitions:[],attribute_defaults:[],attribute_values:[],extended_multiplexing:[]}}function zt(i){return/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(i)}const Ce=["#3b82f6","#22c55e","#f59e0b","#ef4444","#8b5cf6","#06b6d4","#ec4899","#84cc16","#f97316","#6366f1"];function _e(i){return Ce[i%Ce.length]}function j(i,e,t){if(t==="little_endian")return{start:i,end:i+e-1};{const s=i-e+1;return{start:Math.max(0,s),end:i}}}function N(i,e,t,s){return s==="little_endian"?{startMin:0,startMax:Math.max(0,i-t),lenMin:1,lenMax:Math.max(1,i-e)}:{startMin:Math.max(0,t-1),startMax:i-1,lenMin:1,lenMax:Math.min(64,e+1)}}function se(i){const e=[];e.push(`VERSION "${i.version||""}"`),e.push(""),e.push("NS_ :"),e.push(""),i.bit_timing&&i.bit_timing.baudrate>0?i.bit_timing.btr1>0||i.bit_timing.btr2>0?e.push(`BS_: ${i.bit_timing.baudrate} : ${i.bit_timing.btr1},${i.bit_timing.btr2}`):e.push(`BS_: ${i.bit_timing.baudrate}`):e.push("BS_:"),e.push(""),i.nodes.length>0?e.push(`BU_: ${i.nodes.map(t=>t.name).join(" ")}`):e.push("BU_:"),e.push("");for(const t of i.messages){const s=t.is_extended?t.id|2147483648:t.id;e.push(`BO_ ${s} ${t.name}: ${t.dlc} ${t.sender}`);for(const a of t.signals){const n=a.byte_order==="little_endian"?1:0,r=a.is_unsigned?"+":"-";let o="";a.is_multiplexer?o=" M":a.multiplexer_value!==null&&(o=` m${a.multiplexer_value}`);let l="Vector__XXX";a.receivers.type==="nodes"&&a.receivers.nodes.length>0&&(l=a.receivers.nodes.join(","));const p=a.unit||"";e.push(` SG_ ${a.name}${o} : ${a.start_bit}|${a.length}@${n}${r} (${a.factor},${a.offset}) [${a.min}|${a.max}] "${p}" ${l}`)}e.push("")}i.comment&&e.push(`CM_ "${R(i.comment)}" ;`);for(const t of i.nodes)t.comment&&e.push(`CM_ BU_ ${t.name} "${R(t.comment)}" ;`);for(const t of i.messages){const s=t.is_extended?t.id|2147483648:t.id;t.comment&&e.push(`CM_ BO_ ${s} "${R(t.comment)}" ;`);for(const a of t.signals)a.comment&&e.push(`CM_ SG_ ${s} ${a.name} "${R(a.comment)}" ;`)}for(const t of i.attribute_definitions){const s=Nt(t.object_type),a=Pt(t.value_type);e.push(`BA_DEF_ ${s}"${t.name}" ${a} ;`)}for(const t of i.attribute_defaults){const s=typeof t.value=="string"?`"${R(t.value)}"`:String(t.value);e.push(`BA_DEF_DEF_ "${t.name}" ${s} ;`)}for(const t of i.attribute_values){const s=jt(t.target),a=typeof t.value=="string"?`"${R(t.value)}"`:String(t.value);e.push(`BA_ "${t.name}" ${s}${a} ;`)}for(const t of i.value_descriptions){if(t.descriptions.length===0)continue;const s=t.message_id,a=t.descriptions.map(n=>`${n.value} "${R(n.description)}"`).join(" ");e.push(`VAL_ ${s} ${t.signal_name} ${a} ;`)}for(const t of i.extended_multiplexing){const s=t.ranges.map(a=>`${a[0]}-${a[1]}`).join(", ");e.push(`SG_MUL_VAL_ ${t.message_id} ${t.signal_name} ${t.multiplexer_signal} ${s} ;`)}return e.push(""),e.join(`
`)}function Nt(i){switch(i){case"node":return"BU_ ";case"message":return"BO_ ";case"signal":return"SG_ ";default:return""}}function Pt(i){switch(i.type){case"int":return`INT ${i.min} ${i.max}`;case"hex":return`HEX ${i.min} ${i.max}`;case"float":return`FLOAT ${i.min} ${i.max}`;case"string":return"STRING";case"enum":return`ENUM ${i.values.map(e=>`"${R(e)}"`).join(",")}`;default:return"STRING"}}function jt(i){switch(i.type){case"network":return"";case"node":return`BU_ ${i.node_name} `;case"message":return`BO_ ${i.message_id} `;case"signal":return`SG_ ${i.message_id} ${i.signal_name} `;default:return""}}function R(i){return i.replace(/\\/g,"").replace(/\n/g," ").replace(/\r/g,"").replace(/"/g,"'")}class Vt extends HTMLElement{signals=[];selectedName=null;constructor(){super(),this.attachShadow({mode:"open"})}connectedCallback(){this.render()}setSignals(e){this.signals=e,this.render()}setSelected(e){this.selectedName=e,this.render()}render(){if(!this.shadowRoot)return;const e=this.signals.map(t=>{const s=this.selectedName===t.name,a=t.byte_order==="little_endian"?"LE":"BE",n=t.is_unsigned?"+":"±",r=t.unit||"",o=t.is_multiplexer?"M":t.multiplexer_value!==null?`m${t.multiplexer_value}`:"",l=`${t.start_bit}:${t.length}`,p=`${t.min}…${t.max}`;return`
        <tr class="${s?"selected":""}" data-name="${t.name}" title="Factor: ${t.factor}, Offset: ${t.offset}">
          <td class="sig-name">${t.name}</td>
          <td class="sig-mono">${l}</td>
          <td class="sig-center">${a}${n}</td>
          <td class="sig-mono">${p}</td>
          <td>${r}</td>
          <td class="sig-center">${o}</td>
        </tr>
      `}).join("");this.shadowRoot.innerHTML=`
      <style>${L}
        :host { display: block; }
        .sig-name { max-width: 150px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
        .sig-mono { font-family: var(--cv-font-mono); white-space: nowrap; }
        .sig-center { text-align: center; }
      </style>

      ${this.signals.length===0?`
        <div class="cv-empty-message">No signals defined. Click "Add Signal" to create one.</div>
      `:`
        <table class="cv-table cv-table-compact">
          <thead>
            <tr>
              <th>Signal</th>
              <th>Bit:Len</th>
              <th>Type</th>
              <th>Range</th>
              <th>Unit</th>
              <th>Mux</th>
            </tr>
          </thead>
          <tbody>${e}</tbody>
        </table>
      `}
    `,this.shadowRoot.querySelectorAll("tbody tr").forEach(t=>{t.addEventListener("click",()=>{const s=t.dataset.name;s&&this.dispatchEvent(w("signal-select",{name:s}))})})}}customElements.define("cv-signals-table",Vt);class Ht extends HTMLElement{signal=K();originalSignal=K();availableNodes=[];isEditing=!1;parentEditMode=!1;errorMessage=null;static get observedAttributes(){return["data-edit-mode"]}constructor(){super(),this.attachShadow({mode:"open"})}connectedCallback(){this.parentEditMode=this.dataset.editMode==="true",this.render()}attributeChangedCallback(e,t,s){e==="data-edit-mode"&&t!==s&&(this.parentEditMode=s==="true",this.render())}setSignal(e,t){this.signal=e?I(e):K(),this.originalSignal=I(this.signal),this.isEditing=t,this.render()}setAvailableNodes(e){this.availableNodes=e,this.render()}getSignal(){return this.signal}isInEditMode(){return this.isEditing}updateSignalValues(e){if(this.signal={...this.signal,...e},this.shadowRoot&&this.isEditing){if(e.start_bit!==void 0){const t=this.shadowRoot.getElementById("start_bit");t&&(t.value=String(e.start_bit))}if(e.length!==void 0){const t=this.shadowRoot.getElementById("length");t&&(t.value=String(e.length))}}}setError(e){if(this.errorMessage=e,this.shadowRoot&&this.isEditing){const t=this.shadowRoot.querySelector(".cv-error-msg"),s=this.shadowRoot.getElementById("done-btn");t&&(t.textContent=e||"",t.style.display=e?"block":"none"),s&&(s.disabled=!!e,s.style.opacity=e?"0.5":"1",s.style.cursor=e?"not-allowed":"pointer")}}restoreOriginal(){this.signal=I(this.originalSignal),this.errorMessage=null,this.render(),this.dispatchEvent(w("signal-change",this.signal))}render(){this.shadowRoot&&(this.isEditing?this.renderEditMode():this.renderViewMode())}renderViewMode(){if(!this.shadowRoot)return;const e=this.signal.byte_order==="little_endian"?"Little Endian":"Big Endian",t=this.signal.is_unsigned?"Unsigned":"Signed",s=this.signal.unit||"-",a=this.signal.is_multiplexer?"Multiplexer (M)":this.signal.multiplexer_value!==null?`Multiplexed (m${this.signal.multiplexer_value})`:"-",r=this.signal.receivers.type==="nodes"?this.signal.receivers.nodes.join(", "):"None";this.shadowRoot.innerHTML=`
      <style>${L}
        :host { display: block; }
        .header {
          display: flex;
          align-items: center;
          justify-content: space-between;
          margin-bottom: 8px;
          padding-bottom: 6px;
          border-bottom: 1px solid var(--cv-border);
        }
        .signal-name { font-weight: 600; font-size: 15px; color: #fff; }
        .view-container { display: flex; flex-direction: column; gap: 4px; }
        .actions { display: flex; gap: 4px; }
      </style>

      <div class="header">
        <span class="signal-name">${this.signal.name||"(unnamed)"}</span>
        ${this.parentEditMode?`
          <div class="actions">
            <button class="cv-btn primary small" id="edit-btn">Edit</button>
            <button class="cv-btn danger small" id="delete-btn">Delete</button>
          </div>
        `:""}
      </div>

      <div class="view-container">
        ${this.signal.comment?`<div class="cv-signal-comment">${g(this.signal.comment)}</div>`:""}
        <div class="cv-field"><span class="cv-field-label">Start Bit</span><span class="cv-field-value">${this.signal.start_bit}</span></div>
        <div class="cv-field"><span class="cv-field-label">Length</span><span class="cv-field-value">${this.signal.length} bits</span></div>
        <div class="cv-field"><span class="cv-field-label">Byte Order</span><span class="cv-field-value text">${e}</span></div>
        <div class="cv-field"><span class="cv-field-label">Value Type</span><span class="cv-field-value text">${t}</span></div>
        <div class="cv-field"><span class="cv-field-label">Factor</span><span class="cv-field-value">${this.signal.factor}</span></div>
        <div class="cv-field"><span class="cv-field-label">Offset</span><span class="cv-field-value">${this.signal.offset}</span></div>
        <div class="cv-field"><span class="cv-field-label">Min</span><span class="cv-field-value">${this.signal.min}</span></div>
        <div class="cv-field"><span class="cv-field-label">Max</span><span class="cv-field-value">${this.signal.max}</span></div>
        <div class="cv-field"><span class="cv-field-label">Unit</span><span class="cv-field-value text">${s}</span></div>
        <div class="cv-field"><span class="cv-field-label">Multiplexing</span><span class="cv-field-value text">${a}</span></div>
        <div class="cv-field"><span class="cv-field-label">Receivers</span><span class="cv-field-value text">${r}</span></div>
      </div>
    `,this.shadowRoot.getElementById("edit-btn")?.addEventListener("click",()=>{this.originalSignal=I(this.signal),this.isEditing=!0,this.render(),this.dispatchEvent(w("edit-start",{}))}),this.shadowRoot.getElementById("delete-btn")?.addEventListener("click",()=>{this.dispatchEvent(w("signal-delete-request",{name:this.signal.name}))})}renderEditMode(){if(!this.shadowRoot)return;const e=this.signal.receivers.type,t=e==="nodes"?this.signal.receivers.nodes:[];this.shadowRoot.innerHTML=`
      <style>${L}
        :host { display: block; }
        .btn-row { display: flex; gap: 4px; margin-top: 8px; padding-bottom: 4px; }
      </style>

      <div class="cv-form-group-sm">
        <label class="cv-label">Name</label>
        <input type="text" class="cv-input" id="name" value="${this.signal.name}" placeholder="Signal Name">
      </div>

      <div class="cv-form-row-4">
        <div class="cv-form-group cv-form-group-sm">
          <label class="cv-label">Start</label>
          <input type="number" class="cv-input" id="start_bit" value="${this.signal.start_bit}" min="0" max="511">
        </div>
        <div class="cv-form-group cv-form-group-sm">
          <label class="cv-label">Len</label>
          <input type="number" class="cv-input" id="length" value="${this.signal.length}" min="1" max="64">
        </div>
        <div class="cv-form-group cv-form-group-sm">
          <label class="cv-label">Order</label>
          <select class="cv-select" id="byte_order">
            <option value="little_endian" ${this.signal.byte_order==="little_endian"?"selected":""}>LE</option>
            <option value="big_endian" ${this.signal.byte_order==="big_endian"?"selected":""}>BE</option>
          </select>
        </div>
        <div class="cv-form-group cv-form-group-sm">
          <label class="cv-label">Type</label>
          <select class="cv-select" id="is_unsigned">
            <option value="true" ${this.signal.is_unsigned?"selected":""}>U</option>
            <option value="false" ${this.signal.is_unsigned?"":"selected"}>S</option>
          </select>
        </div>
      </div>

      <div class="cv-form-row">
        <div class="cv-form-group cv-form-group-sm">
          <label class="cv-label">Factor</label>
          <input type="number" class="cv-input" id="factor" value="${this.signal.factor}" step="any">
        </div>
        <div class="cv-form-group cv-form-group-sm">
          <label class="cv-label">Offset</label>
          <input type="number" class="cv-input" id="offset" value="${this.signal.offset}" step="any">
        </div>
      </div>

      <div class="cv-form-row">
        <div class="cv-form-group cv-form-group-sm">
          <label class="cv-label">Min</label>
          <input type="number" class="cv-input" id="min" value="${this.signal.min}" step="any">
        </div>
        <div class="cv-form-group cv-form-group-sm">
          <label class="cv-label">Max</label>
          <input type="number" class="cv-input" id="max" value="${this.signal.max}" step="any">
        </div>
      </div>

      <div class="cv-form-row">
        <div class="cv-form-group cv-form-group-sm">
          <label class="cv-label">Unit</label>
          <input type="text" class="cv-input" id="unit" value="${this.signal.unit||""}" placeholder="">
        </div>
        <div class="cv-form-group cv-form-group-sm">
          <label class="cv-label">Receivers</label>
          <select class="cv-select" id="receivers_type">
            <option value="none" ${e==="none"?"selected":""}>None</option>
            <option value="nodes" ${e==="nodes"?"selected":""}>Nodes</option>
          </select>
        </div>
      </div>

      <div class="cv-form-group-sm receivers-nodes" style="display: ${e==="nodes"?"block":"none"}">
        <div style="display: flex; flex-wrap: wrap; gap: 4px;">
          ${this.availableNodes.map(s=>`
            <label class="cv-checkbox-group" style="font-size: 11px;">
              <input type="checkbox" class="cv-checkbox receiver-node" value="${s}" ${t.includes(s)?"checked":""} style="width: 14px; height: 14px;">
              <span>${s}</span>
            </label>
          `).join("")}
        </div>
      </div>

      <div class="cv-form-row">
        <div class="cv-form-group cv-form-group-sm">
          <label class="cv-checkbox-group" style="font-size: 12px;">
            <input type="checkbox" class="cv-checkbox" id="is_multiplexer" ${this.signal.is_multiplexer?"checked":""} style="width: 14px; height: 14px;">
            <span>Mux Switch</span>
          </label>
        </div>
        <div class="cv-form-group cv-form-group-sm">
          <label class="cv-label">Mux Val</label>
          <input type="number" class="cv-input" id="multiplexer_value" value="${this.signal.multiplexer_value??""}" placeholder="-" min="0">
        </div>
      </div>

      <div class="cv-form-group-sm">
        <label class="cv-label">Comment</label>
        <input type="text" class="cv-input" id="comment" value="${this.signal.comment||""}" placeholder="Optional description">
      </div>

      <div class="cv-error-msg" style="display: ${this.errorMessage?"block":"none"}">${this.errorMessage||""}</div>

      <div class="btn-row">
        <button class="cv-btn success small" id="done-btn" ${this.errorMessage?'disabled style="opacity:0.5;cursor:not-allowed"':""}>Done</button>
        <button class="cv-btn warning small" id="restore-btn">Restore</button>
        <button class="cv-btn small" id="cancel-btn">Cancel</button>
      </div>
    `,this.setupEditListeners()}setupEditListeners(){if(!this.shadowRoot)return;["name","start_bit","length","factor","offset","min","max","unit","multiplexer_value","comment"].forEach(a=>{this.shadowRoot.getElementById(a)?.addEventListener("input",()=>this.updateSignalFromInputs())}),["byte_order","is_unsigned","receivers_type"].forEach(a=>{const n=this.shadowRoot.getElementById(a);n?.addEventListener("change",()=>{if(this.updateSignalFromInputs(),a==="receivers_type"){const r=this.shadowRoot.querySelector(".receivers-nodes");r&&(r.style.display=n.value==="nodes"?"block":"none")}})}),this.shadowRoot.getElementById("is_multiplexer")?.addEventListener("change",()=>this.updateSignalFromInputs()),this.shadowRoot.querySelectorAll(".receiver-node").forEach(a=>{a.addEventListener("change",()=>this.updateSignalFromInputs())}),this.shadowRoot.getElementById("done-btn")?.addEventListener("click",()=>{this.errorMessage||(this.updateSignalFromInputs(),this.isEditing=!1,this.errorMessage=null,this.render(),this.dispatchEvent(w("edit-done",this.signal)))}),this.shadowRoot.getElementById("restore-btn")?.addEventListener("click",()=>{this.restoreOriginal()}),this.shadowRoot.getElementById("cancel-btn")?.addEventListener("click",()=>{this.signal=I(this.originalSignal),this.isEditing=!1,this.errorMessage=null,this.render(),this.dispatchEvent(w("edit-cancel",{}))})}updateSignalFromInputs(){if(!this.shadowRoot)return;const e=o=>this.shadowRoot.getElementById(o)?.value||"",t=o=>this.shadowRoot.getElementById(o)?.checked||!1,s=e("multiplexer_value"),a=s!==""?parseInt(s,10):null,n=e("receivers_type");let r;if(n==="nodes"){const o=[];this.shadowRoot.querySelectorAll(".receiver-node:checked").forEach(l=>{o.push(l.value)}),r={type:"nodes",nodes:o}}else r={type:"none"};this.signal={name:e("name"),start_bit:parseInt(e("start_bit"),10)||0,length:parseInt(e("length"),10)||1,byte_order:e("byte_order"),is_unsigned:e("is_unsigned")==="true",factor:parseFloat(e("factor"))||1,offset:parseFloat(e("offset"))||0,min:parseFloat(e("min"))||0,max:parseFloat(e("max"))||0,unit:e("unit")||null,receivers:r,is_multiplexer:t("is_multiplexer"),multiplexer_value:a,comment:e("comment")||null},this.dispatchEvent(w("signal-change",this.signal))}}customElements.define("cv-signal-editor",Ht);class Ot extends HTMLElement{messages=[];selectedId=null;selectedIsExtended=!1;delegatedHandler=null;constructor(){super()}connectedCallback(){this.setupEventDelegation()}disconnectedCallback(){this.removeEventDelegation()}setupEventDelegation(){const e=this.querySelector(".cv-table-wrap");!e||this.delegatedHandler||(this.delegatedHandler=t=>{const a=t.target.closest("tr.clickable");if(a?.dataset.id){const n=parseInt(a.dataset.id,10),r=a.dataset.extended==="true";this.dispatchEvent(new CustomEvent("message-select",{detail:{id:n,isExtended:r},bubbles:!0}))}},e.addEventListener("click",this.delegatedHandler))}removeEventDelegation(){if(!this.delegatedHandler)return;const e=this.querySelector(".cv-table-wrap");e&&e.removeEventListener("click",this.delegatedHandler),this.delegatedHandler=null}setMessages(e){this.messages=e,this.render()}setSelected(e,t){this.selectedId=e,this.selectedIsExtended=t,this.updateSelection()}render(){let e=this.querySelector(".cv-table-wrap");e||(e=document.createElement("div"),e.className="cv-table-wrap",this.appendChild(e));const t=this.messages.map(s=>{const a=this.selectedId===s.id&&this.selectedIsExtended===s.is_extended,n=P(s.id,s.is_extended);return`
        <tr class="clickable ${a?"selected":""}"
            data-id="${s.id}"
            data-extended="${s.is_extended}">
          <td class="cv-cell-id">${n}</td>
          <td class="cv-cell-name">${s.name||"(unnamed)"}</td>
          <td class="cv-cell-dim">${s.dlc}</td>
          <td class="cv-cell-dim">${s.signals.length}</td>
          <td class="cv-cell-dim">${s.sender}</td>
        </tr>
      `}).join("");e.innerHTML=`
      <table class="cv-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>DLC</th>
            <th>Signals</th>
            <th>Sender</th>
          </tr>
        </thead>
        <tbody>${t}</tbody>
      </table>
    `,this.removeEventDelegation(),this.setupEventDelegation()}updateSelection(){const e=this.querySelector("tbody");e&&e.querySelectorAll("tr").forEach(t=>{const s=parseInt(t.dataset.id||"-1",10),a=t.dataset.extended==="true",n=s===this.selectedId&&a===this.selectedIsExtended;t.classList.toggle("selected",n)})}}customElements.define("cv-messages-list",Ot);class Ut extends HTMLElement{message=Q();originalMessage=Q();availableNodes=[];frames=[];selectedSignal=null;editingSignal=null;isAddingSignal=!1;isEditingSignal=!1;isEditingMessage=!1;isNewMessage=!1;parentEditMode=!1;static get observedAttributes(){return["data-edit-mode"]}constructor(){super(),this.attachShadow({mode:"open"})}connectedCallback(){this.parentEditMode=this.dataset.editMode==="true",this.render()}attributeChangedCallback(e,t,s){e==="data-edit-mode"&&t!==s&&(this.parentEditMode=s==="true",this.render())}setMessage(e,t){this.message=e?I(e):Q(),this.originalMessage=I(this.message),this.selectedSignal=null,this.editingSignal=null,this.isAddingSignal=!1,this.isNewMessage=t,this.isEditingMessage=t,this.render()}setAvailableNodes(e){this.availableNodes=e,this.render()}setFrames(e){this.frames=e}getMessage(){return this.message}isInEditMode(){return this.isEditingMessage}renderMessageViewMode(e){const t=this.message.id;return`
      <div class="cv-card-header cv-msg-card-header">
        <div class="cv-msg-header">
          <div class="cv-msg-header-info">
            <span class="cv-msg-title">${this.message.name||"(unnamed)"}</span>
            <span class="cv-msg-id">${e} (${t})</span>
            <span class="cv-msg-meta">DLC: ${this.message.dlc}</span>
            ${this.message.sender?`<span class="cv-msg-meta">TX: ${this.message.sender}</span>`:""}
            ${this.message.is_extended?'<span class="cv-msg-meta">Extended</span>':""}
            ${this.message.comment?`<span class="cv-msg-meta dimmed">${g(this.message.comment)}</span>`:""}
          </div>
          ${this.parentEditMode?`
            <div class="cv-msg-actions">
              <button class="cv-btn primary small" id="edit-msg-btn">Edit</button>
              <button class="cv-btn danger small" id="delete-msg-btn">Delete</button>
            </div>
          `:""}
        </div>
        ${this.renderBitLayout()}
      </div>
    `}renderMessageEditMode(e){return`
      <div class="cv-card-header cv-msg-card-header">
        <div class="cv-msg-header">
          <div class="cv-form-group" style="flex: 1; margin-bottom: 0; margin-right: 16px;">
            <label class="cv-label">Name</label>
            <input type="text" class="cv-input" id="msg_name" value="${this.message.name}" placeholder="Message Name">
          </div>
          <div class="cv-msg-actions" style="align-self: flex-end;">
            <button class="cv-btn success small" id="done-msg-btn">Done</button>
            <button class="cv-btn small" id="cancel-msg-btn">Cancel</button>
          </div>
        </div>

        <div class="cv-form-row">
          <div class="cv-form-group">
            <label class="cv-label">Message ID <span class="cv-id-display">(${e})</span></label>
            <input type="number" class="cv-input" id="msg_id" value="${this.message.id}" min="0" max="536870911">
          </div>
          <div class="cv-form-group">
            <label class="cv-label">DLC</label>
            <select class="cv-select" id="msg_dlc">
              ${[0,1,2,3,4,5,6,7,8,12,16,20,24,32,48,64].map(t=>`<option value="${t}" ${this.message.dlc===t?"selected":""}>${t}</option>`).join("")}
            </select>
          </div>
        </div>

        <div class="cv-form-row">
          <div class="cv-form-group">
            <label class="cv-label">Sender</label>
            <select class="cv-select" id="msg_sender">
              <option value="Vector__XXX" ${this.message.sender==="Vector__XXX"?"selected":""}>Vector__XXX</option>
              ${this.availableNodes.map(t=>`<option value="${t}" ${this.message.sender===t?"selected":""}>${t}</option>`).join("")}
            </select>
          </div>
          <div class="cv-form-group">
            <div class="cv-checkbox-group" style="margin-top: 20px;">
              <input type="checkbox" class="cv-checkbox" id="msg_extended" ${this.message.is_extended?"checked":""}>
              <span>Extended ID (29-bit)</span>
            </div>
          </div>
        </div>

        <div class="cv-form-group">
          <label class="cv-label">Comment</label>
          <textarea class="cv-textarea" id="msg_comment" rows="2" placeholder="Optional description">${this.message.comment||""}</textarea>
        </div>

        ${this.renderBitLayout()}
      </div>
    `}getActiveSignals(){const t=this.editingSignal?.multiplexer_value;return this.message.signals.filter(s=>!!(s.is_multiplexer||s.multiplexer_value===null||t!==null&&s.multiplexer_value===t))}renderBitLayout(){const e=this.message.dlc*8;if(e===0)return"";const t=this.editingSignal,s=t?.start_bit??0,a=t?.length??1,n=this.getActiveSignals(),r=c=>j(c.start_bit,c.length,c.byte_order),o=c=>{const d=r(c);for(const h of n){if(h.name===c.name)continue;const u=r(h);if(d.start<=u.end&&u.start<=d.end)return!0}return!1},l=n.map((c,d)=>{const h=t&&c.name===t.name,u=o(c),v=r(c),m=v.start/e*100,y=c.length/e*100,S=u?"#ef4444":h?"#3b82f6":_e(d),T=h?1:.5,A=["cv-bit-segment",h?"current":"",u?"overlap":""].filter(Boolean).join(" "),C=c.byte_order==="big_endian"?"BE":"LE";return`<div class="${A}"
                   style="left: ${m}%; width: ${y}%; background: ${S}; opacity: ${T};"
                   title="${c.name} (${C}): bits ${v.start}-${v.end}${u?" (OVERLAP!)":""}">
                ${y>8?c.name:""}
              </div>`}).join(""),p=[];for(let c=0;c<=Math.min(8,this.message.dlc);c++)p.push(`<span>${c*8}</span>`);return this.message.dlc>8&&p.push(`<span>${e}</span>`),`
      <div class="cv-bit-layout">
        <div class="cv-bit-layout-header">
          <span>Bit Layout (${e} bits)</span>
          <span>${t?.name||""}: ${s} - ${s+a-1}</span>
        </div>
        <div class="cv-bit-bar">
          ${l}
        </div>
        <div class="cv-bit-markers">
          ${p.join("")}
        </div>
        ${this.isAddingSignal||this.isEditingSignal?(()=>{const c=t?.byte_order??"little_endian",d=N(e,s,a,c);return`
          <div class="cv-bit-sliders">
            <div class="cv-bit-slider-group">
              <div class="cv-bit-slider-label">
                <span>Start Bit</span>
                <span class="cv-bit-slider-value" id="start-bit-value">${s}</span>
              </div>
              <input type="range" class="cv-bit-slider" id="start-bit-slider"
                     min="${d.startMin}" max="${d.startMax}" value="${s}">
            </div>
            <div class="cv-bit-slider-group">
              <div class="cv-bit-slider-label">
                <span>Length</span>
                <span class="cv-bit-slider-value" id="length-value">${a}</span>
              </div>
              <input type="range" class="cv-bit-slider" id="length-slider"
                     min="${d.lenMin}" max="${d.lenMax}" value="${a}">
            </div>
          </div>
        `})():""}
      </div>
    `}updateBitBar(){if(!this.shadowRoot||!this.editingSignal)return;const e=this.message.dlc*8;if(e===0)return;const t=this.editingSignal.start_bit,s=this.editingSignal.length,a=this.shadowRoot.querySelector(".cv-bit-layout-header span:last-child");a&&(a.textContent=`${this.editingSignal.name||""}: ${t} - ${t+s-1}`);const n=this.shadowRoot.querySelector(".cv-bit-bar");if(n){const r=this.getActiveSignals(),o=this.editingSignal.byte_order,l=(d,h,u)=>j(d,h,u),p=(d,h,u,v)=>{const m=l(d,h,u);for(const y of r){if(y.name===v)continue;const S=y.name===this.editingSignal.name,T=S?t:y.start_bit,A=S?s:y.length,C=S?o:y.byte_order,D=l(T,A,C);if(m.start<=D.end&&D.start<=m.end)return!0}return!1},c=r.map((d,h)=>{const u=d.name===this.editingSignal.name,v=u?t:d.start_bit,m=u?s:d.length,y=u?o:d.byte_order,S=l(v,m,y),T=S.start/e*100,A=m/e*100,C=p(v,m,y,d.name),D=C?"#ef4444":u?"#3b82f6":_e(h),Qe=u?1:.5,We=y==="big_endian"?"BE":"LE";return`<div class="${["cv-bit-segment",u?"current":"",C?"overlap":""].filter(Boolean).join(" ")}"
                     style="left: ${T}%; width: ${A}%; background: ${D}; opacity: ${Qe};"
                     title="${d.name} (${We}): bits ${S.start}-${S.end}${C?" (OVERLAP!)":""}">
                  ${A>8?d.name:""}
                </div>`}).join("");if(this.isAddingSignal&&this.editingSignal){const d=l(t,s,o),h=d.start/e*100,u=s/e*100,v=p(t,s,o,""),m=v?"#ef4444":"#3b82f6",y=o==="big_endian"?"BE":"LE",T=`<div class="${["cv-bit-segment","current",v?"overlap":""].filter(Boolean).join(" ")}"
                              style="left: ${h}%; width: ${u}%; background: ${m}; opacity: 1;"
                              title="New (${y}): bits ${d.start}-${d.end}${v?" (OVERLAP!)":""}">
                           ${u>8?this.editingSignal.name||"New":""}
                         </div>`;n.innerHTML=c+T}else n.innerHTML=c}}setupSliderListeners(){if(!this.shadowRoot)return;const e=this.shadowRoot.getElementById("start-bit-slider"),t=this.shadowRoot.getElementById("length-slider"),s=this.message.dlc*8,a=()=>{if(!this.editingSignal||!e||!t)return;const n=N(s,this.editingSignal.start_bit,this.editingSignal.length,this.editingSignal.byte_order);e.min=String(n.startMin),e.max=String(n.startMax),t.min=String(n.lenMin),t.max=String(n.lenMax)};e?.addEventListener("input",()=>{const n=parseInt(e.value,10);if(this.editingSignal){this.editingSignal.start_bit=n,this.shadowRoot.getElementById("start-bit-value").textContent=String(n),a();const r=N(s,n,this.editingSignal.length,this.editingSignal.byte_order);this.editingSignal.length>r.lenMax&&(this.editingSignal.length=r.lenMax,t.value=String(r.lenMax),this.shadowRoot.getElementById("length-value").textContent=String(r.lenMax));const o=this.shadowRoot.querySelector("cv-signal-editor");o?.updateSignalValues({start_bit:this.editingSignal.start_bit,length:this.editingSignal.length}),this.updateBitBar(),this.validateSignalAndSetError(o)}}),t?.addEventListener("input",()=>{const n=parseInt(t.value,10);if(this.editingSignal){this.editingSignal.length=n,this.shadowRoot.getElementById("length-value").textContent=String(n),a();const r=N(s,this.editingSignal.start_bit,n,this.editingSignal.byte_order);this.editingSignal.start_bit<r.startMin?(this.editingSignal.start_bit=r.startMin,e.value=String(r.startMin),this.shadowRoot.getElementById("start-bit-value").textContent=String(r.startMin)):this.editingSignal.start_bit>r.startMax&&(this.editingSignal.start_bit=r.startMax,e.value=String(r.startMax),this.shadowRoot.getElementById("start-bit-value").textContent=String(r.startMax));const o=this.shadowRoot.querySelector("cv-signal-editor");o?.updateSignalValues({start_bit:this.editingSignal.start_bit,length:this.editingSignal.length}),this.updateBitBar(),this.validateSignalAndSetError(o)}})}render(){if(!this.shadowRoot)return;const e=`0x${this.message.id.toString(16).toUpperCase()}`;this.shadowRoot.innerHTML=`
      <style>${L}
        :host {
          display: flex;
          flex-direction: column;
          gap: 0;
          padding: 12px;
          flex: 1;
          min-height: 0;
          overflow-y: auto;
        }
      </style>

      ${this.isEditingMessage?this.renderMessageEditMode(e):this.renderMessageViewMode(e)}

      <div class="cv-signals-layout">
        <div class="cv-signals-table-container">
          <div class="cv-signals-table-header">
            <span>Signals (${this.message.signals.length})</span>
            ${this.parentEditMode?'<button class="cv-btn accent small" id="add-signal-btn">+ Add</button>':""}
          </div>
          <cv-signals-table></cv-signals-table>
        </div>
        ${this.isAddingSignal||this.selectedSignal?`
          <div class="cv-signal-editor-panel">
            <cv-signal-editor data-edit-mode="${this.parentEditMode}"></cv-signal-editor>
          </div>
        `:""}
      </div>
    `,this.setupEventListeners(),this.updateChildComponents()}setupEventListeners(){if(!this.shadowRoot)return;this.shadowRoot.getElementById("edit-msg-btn")?.addEventListener("click",()=>{this.isEditingMessage=!0,this.render()}),this.shadowRoot.getElementById("delete-msg-btn")?.addEventListener("click",()=>{this.dispatchEvent(new CustomEvent("message-delete-request",{bubbles:!0,composed:!0}))}),this.shadowRoot.getElementById("done-msg-btn")?.addEventListener("click",()=>{if(!this.message.name){alert("Message name is required");return}this.isEditingMessage=!1,this.originalMessage=I(this.message),this.notifyChange(),this.dispatchEvent(new CustomEvent("message-edit-done",{detail:this.message,bubbles:!0,composed:!0})),this.render()}),this.shadowRoot.getElementById("cancel-msg-btn")?.addEventListener("click",()=>{this.isNewMessage?this.dispatchEvent(new CustomEvent("message-edit-cancel",{bubbles:!0,composed:!0})):(this.message=I(this.originalMessage),this.isEditingMessage=!1,this.render())});const e=this.shadowRoot.getElementById("msg_name"),t=this.shadowRoot.getElementById("msg_id"),s=this.shadowRoot.getElementById("msg_dlc"),a=this.shadowRoot.getElementById("msg_sender"),n=this.shadowRoot.getElementById("msg_extended");e?.addEventListener("input",()=>{this.message.name=e.value}),t?.addEventListener("input",()=>{this.message.id=parseInt(t.value,10)||0;const c=`0x${this.message.id.toString(16).toUpperCase()}`,d=this.shadowRoot.querySelector(".cv-id-display");if(d&&(d.textContent=`(${c})`),this.isNewMessage&&this.frames.length>0){const h=be(this.frames,this.message.id,this.message.is_extended);h!==null&&h!==this.message.dlc&&(this.message.dlc=h,s&&(s.value=String(h)))}}),s?.addEventListener("change",()=>{this.message.dlc=parseInt(s.value,10)}),a?.addEventListener("change",()=>{this.message.sender=a.value}),n?.addEventListener("change",()=>{if(this.message.is_extended=n.checked,this.isNewMessage&&this.frames.length>0){const c=be(this.frames,this.message.id,this.message.is_extended);c!==null&&c!==this.message.dlc&&(this.message.dlc=c,s&&(s.value=String(c)))}});const r=this.shadowRoot.getElementById("msg_comment");r?.addEventListener("input",()=>{this.message.comment=r.value||null}),this.setupSliderListeners(),this.shadowRoot.getElementById("add-signal-btn")?.addEventListener("click",()=>{this.isAddingSignal=!0,this.selectedSignal=null,this.editingSignal=K(),this.render()}),this.shadowRoot.querySelector("cv-signals-table")?.addEventListener("signal-select",(c=>{const d=c.detail.name;this.selectedSignal===d?(this.selectedSignal=null,this.editingSignal=null,this.isEditingSignal=!1):(this.selectedSignal=d,this.editingSignal=this.message.signals.find(h=>h.name===d)||null,this.isEditingSignal=!1),this.isAddingSignal=!1,this.render()}));const p=this.shadowRoot.querySelector("cv-signal-editor");p?.addEventListener("edit-start",(()=>{this.isEditingSignal=!0;const c=this.shadowRoot.querySelector(".cv-bit-layout");if(c&&!c.querySelector(".cv-bit-sliders")){const d=this.message.dlc*8,h=this.editingSignal?.start_bit??0,u=this.editingSignal?.length??1,v=this.editingSignal?.byte_order??"little_endian",m=N(d,h,u,v),y=`
          <div class="cv-bit-sliders">
            <div class="cv-bit-slider-group">
              <div class="cv-bit-slider-label">
                <span>Start Bit</span>
                <span class="cv-bit-slider-value" id="start-bit-value">${h}</span>
              </div>
              <input type="range" class="cv-bit-slider" id="start-bit-slider"
                     min="${m.startMin}" max="${m.startMax}" value="${h}">
            </div>
            <div class="cv-bit-slider-group">
              <div class="cv-bit-slider-label">
                <span>Length</span>
                <span class="cv-bit-slider-value" id="length-value">${u}</span>
              </div>
              <input type="range" class="cv-bit-slider" id="length-slider"
                     min="${m.lenMin}" max="${m.lenMax}" value="${u}">
            </div>
          </div>
        `;c.insertAdjacentHTML("beforeend",y),this.setupSliderListeners()}})),p?.addEventListener("signal-change",(c=>{const d=c.detail;if(this.editingSignal){this.editingSignal={...this.editingSignal,...d};const h=this.shadowRoot.getElementById("start-bit-slider"),u=this.shadowRoot.getElementById("length-slider"),v=this.message.dlc*8;if(h&&u){const m=N(v,d.start_bit,d.length,d.byte_order);h.min=String(m.startMin),h.max=String(m.startMax),u.min=String(m.lenMin),u.max=String(m.lenMax),h.value=String(d.start_bit),u.value=String(d.length);const y=this.shadowRoot.getElementById("start-bit-value"),S=this.shadowRoot.getElementById("length-value");y&&(y.textContent=String(d.start_bit)),S&&(S.textContent=String(d.length))}this.updateBitBar(),this.validateSignalAndSetError(p)}})),p?.addEventListener("edit-done",(c=>{const d=c.detail;if(!d.name){alert("Signal name is required");return}const h=this.message.dlc*8;if(d.start_bit+d.length>h){alert(`Signal extends beyond message size (${h} bits). Reduce start bit or length.`);return}const u=this.isAddingSignal?null:this.selectedSignal,v=this.findOverlappingSignal(d,u);if(v){alert(`Signal "${d.name}" overlaps with "${v.name}" (bits ${v.start_bit}-${v.start_bit+v.length-1})`);return}if(this.isAddingSignal){if(this.message.signals.some(m=>m.name===d.name)){alert(`Signal "${d.name}" already exists`);return}this.message.signals.push(d),this.selectedSignal=d.name}else if(this.selectedSignal){const m=this.message.signals.findIndex(y=>y.name===this.selectedSignal);if(m>=0){if(d.name!==this.selectedSignal&&this.message.signals.some(y=>y.name===d.name)){alert(`Signal "${d.name}" already exists`);return}this.message.signals[m]=d,this.selectedSignal=d.name}}this.isAddingSignal=!1,this.isEditingSignal=!1,this.editingSignal=d,this.notifyChange(),this.render()})),p?.addEventListener("edit-cancel",(()=>{if(this.isEditingSignal=!1,this.isAddingSignal)this.isAddingSignal=!1,this.selectedSignal=null,this.editingSignal=null;else if(this.selectedSignal){const c=this.message.signals.find(d=>d.name===this.selectedSignal);this.editingSignal=c?I(c):null}this.render()})),p?.addEventListener("signal-delete-request",(c=>{const d=c.detail.name;this.message.signals=this.message.signals.filter(h=>h.name!==d),this.selectedSignal=null,this.editingSignal=null,this.notifyChange(),this.render()}))}updateChildComponents(){if(!this.shadowRoot)return;const e=this.shadowRoot.querySelector("cv-signals-table");e&&(e.setSignals(this.message.signals),e.setSelected(this.selectedSignal));const t=this.shadowRoot.querySelector("cv-signal-editor");t&&this.editingSignal&&(t.setSignal(this.editingSignal,this.isAddingSignal),t.setAvailableNodes(this.availableNodes))}findOverlappingSignal(e,t){const s=j(e.start_bit,e.length,e.byte_order);for(const a of this.message.signals){if(t&&a.name===t||e.multiplexer_value!==null&&a.multiplexer_value!==null&&e.multiplexer_value!==a.multiplexer_value)continue;const n=j(a.start_bit,a.length,a.byte_order);if(s.start<=n.end&&n.start<=s.end)return a}return null}validateSignalAndSetError(e){if(!e||!this.editingSignal)return;const t=this.message.dlc*8,s=this.editingSignal,a=j(s.start_bit,s.length,s.byte_order);if(a.start<0||a.end>=t){e.setError(`Signal exceeds message bounds (0-${t-1} bits)`);return}const n=this.isAddingSignal?null:this.selectedSignal,r=this.findOverlappingSignal(s,n);if(r){e.setError(`Overlaps with "${r.name}"`);return}e.setError(null)}notifyChange(){this.dispatchEvent(new CustomEvent("message-change",{detail:this.message,bubbles:!0,composed:!0}))}}customElements.define("cv-message-editor",Ut);class Xt extends HTMLElement{nodes=[];editingNodeName=null;constructor(){super(),this.attachShadow({mode:"open"})}connectedCallback(){this.render()}setNodes(e){this.nodes=e.map(t=>({...t})),this.render()}getNodes(){return this.nodes}render(){if(!this.shadowRoot)return;const e=this.nodes.map(t=>this.editingNodeName===t.name?`
          <div class="cv-node-card editing">
            <div class="cv-node-card-header">
              <span class="cv-node-name">${g(t.name)}</span>
              <button class="cv-node-remove" data-node="${g(t.name)}">&times;</button>
            </div>
            <div class="cv-node-card-body">
              <textarea class="cv-textarea cv-node-comment-input" id="node-comment-${g(t.name)}"
                        rows="2" placeholder="Optional comment...">${t.comment||""}</textarea>
              <div class="cv-node-actions">
                <button class="cv-btn small success" data-save-node="${g(t.name)}">Save</button>
                <button class="cv-btn small" data-cancel-node="${g(t.name)}">Cancel</button>
              </div>
            </div>
          </div>
        `:`
        <div class="cv-node-card" data-click-node="${g(t.name)}">
          <div class="cv-node-card-header">
            <span class="cv-node-name">${g(t.name)}</span>
            <button class="cv-node-remove" data-node="${g(t.name)}">&times;</button>
          </div>
          ${t.comment?`<div class="cv-node-comment">${g(t.comment)}</div>`:""}
        </div>
      `).join("");this.shadowRoot.innerHTML=`
      <style>${L}
        :host { display: block; }
        .cv-nodes-grid {
          display: flex;
          flex-wrap: wrap;
          gap: 8px;
          margin-bottom: 12px;
        }
        .cv-node-card {
          display: flex;
          flex-direction: column;
          padding: 8px 12px;
          background: var(--cv-bg-elevated);
          border: 1px solid var(--cv-border);
          border-radius: var(--cv-radius);
          min-width: 120px;
          cursor: pointer;
          transition: border-color 0.15s;
        }
        .cv-node-card:hover {
          border-color: var(--cv-accent);
        }
        .cv-node-card.editing {
          cursor: default;
          border-color: var(--cv-accent);
          min-width: 250px;
        }
        .cv-node-card-header {
          display: flex;
          align-items: center;
          justify-content: space-between;
          gap: 8px;
        }
        .cv-node-name {
          font-weight: 500;
          color: var(--cv-text);
        }
        .cv-node-comment {
          font-size: 0.8rem;
          color: var(--cv-text-muted);
          font-style: italic;
          margin-top: 4px;
          padding-top: 4px;
          border-top: 1px dashed var(--cv-border);
        }
        .cv-node-card-body {
          margin-top: 8px;
        }
        .cv-node-comment-input {
          width: 100%;
          min-height: 50px;
        }
        .cv-node-actions {
          display: flex;
          gap: 6px;
          margin-top: 8px;
        }
      </style>

      ${this.nodes.length===0?`
        <p class="cv-empty-message" style="text-align: left; padding: 0; margin-bottom: 12px; font-style: italic;">No nodes defined. Add ECU/node names below.</p>
      `:`
        <div class="cv-nodes-grid">${e}</div>
      `}

      <div class="cv-add-node-form">
        <input type="text" class="cv-input" id="new-node-input" placeholder="Enter node name (e.g., ECM, TCM)">
        <button class="cv-btn primary" id="add-node-btn">Add Node</button>
      </div>
    `,this.setupEventListeners()}setupEventListeners(){if(!this.shadowRoot)return;this.shadowRoot.querySelectorAll(".cv-node-remove").forEach(a=>{a.addEventListener("click",n=>{n.stopPropagation();const r=a.dataset.node;r&&(this.nodes=this.nodes.filter(o=>o.name!==r),this.editingNodeName===r&&(this.editingNodeName=null),this.notifyChange(),this.render())})}),this.shadowRoot.querySelectorAll("[data-click-node]").forEach(a=>{a.addEventListener("click",()=>{const n=a.dataset.clickNode;n&&this.editingNodeName!==n&&(this.editingNodeName=n,this.render())})}),this.shadowRoot.querySelectorAll("[data-save-node]").forEach(a=>{a.addEventListener("click",()=>{const n=a.dataset.saveNode;if(n){const r=this.shadowRoot.getElementById(`node-comment-${n}`),o=this.nodes.find(l=>l.name===n);o&&r&&(o.comment=r.value.trim()||null,this.editingNodeName=null,this.notifyChange(),this.render())}})}),this.shadowRoot.querySelectorAll("[data-cancel-node]").forEach(a=>{a.addEventListener("click",()=>{this.editingNodeName=null,this.render()})});const e=this.shadowRoot.getElementById("new-node-input"),t=this.shadowRoot.getElementById("add-node-btn"),s=()=>{const a=e.value.trim();if(a&&!this.nodes.some(n=>n.name===a)){if(!zt(a)){alert("Node name must start with a letter or underscore and contain only alphanumeric characters and underscores.");return}this.nodes.push({name:a,comment:null}),e.value="",this.notifyChange(),this.render()}else this.nodes.some(n=>n.name===a)&&alert(`Node "${a}" already exists.`)};t?.addEventListener("click",s),e?.addEventListener("keypress",a=>{a.key==="Enter"&&s()})}notifyChange(){this.dispatchEvent(w("nodes-change",{nodes:this.nodes}))}}customElements.define("cv-nodes-editor",Xt);class Zt extends HTMLElement{valueDescriptions=[];messages=[];expandedKey=null;availableSignals=[];showingCreateForm=!1;constructor(){super(),this.attachShadow({mode:"open"})}connectedCallback(){this.render()}setValueDescriptions(e){this.valueDescriptions=e.map(t=>({...t,descriptions:t.descriptions.map(s=>({...s}))})),this.render()}setMessages(e){this.messages=e,this.render()}getValueDescriptions(){return this.valueDescriptions}getKey(e){return`${e.message_id}:${e.signal_name}`}render(){if(!this.shadowRoot)return;this.availableSignals=this.getAvailableSignals();const e=this.valueDescriptions.map(t=>{const s=this.getKey(t),a=this.expandedKey===s,r=this.messages.find(o=>o.id===t.message_id)?.name||P(t.message_id,!1);return`
        <div class="cv-vd-item ${a?"expanded":""}" data-key="${s}">
          <div class="cv-vd-header" data-toggle="${s}">
            <span class="cv-vd-expand">${a?"▼":"▶"}</span>
            <span class="cv-vd-signal">${g(t.signal_name)}</span>
            <span class="cv-vd-message">${g(r)}</span>
            <span class="cv-vd-count">${t.descriptions.length} value${t.descriptions.length!==1?"s":""}</span>
            <button class="cv-btn-icon delete-btn" data-delete="${s}" title="Delete">&times;</button>
          </div>
          ${a?this.renderEntries(t,s):this.renderChips(t)}
        </div>
      `}).join("");this.shadowRoot.innerHTML=`
      <style>${L}
        :host { display: block; }
        .cv-vd-list {
          display: flex;
          flex-direction: column;
          gap: 8px;
        }
        .cv-vd-item {
          background: var(--cv-bg-elevated);
          border: 1px solid var(--cv-border);
          border-radius: var(--cv-radius);
          overflow: hidden;
        }
        .cv-vd-item.expanded {
          border-color: var(--cv-accent);
        }
        .cv-vd-header {
          display: flex;
          align-items: center;
          gap: 8px;
          padding: 10px 12px;
          cursor: pointer;
          user-select: none;
        }
        .cv-vd-header:hover {
          background: var(--cv-bg-hover);
        }
        .cv-vd-expand {
          font-size: 0.7rem;
          color: var(--cv-text-muted);
          width: 12px;
        }
        .cv-vd-signal {
          font-weight: 600;
          color: var(--cv-text);
        }
        .cv-vd-message {
          font-size: 0.85rem;
          color: var(--cv-text-muted);
        }
        .cv-vd-count {
          font-size: 0.8rem;
          color: var(--cv-text-muted);
          margin-left: auto;
          margin-right: 8px;
        }
        .cv-vd-chips {
          padding: 0 12px 10px 32px;
          display: flex;
          flex-wrap: wrap;
          gap: 6px;
        }
        .cv-vd-chip {
          padding: 2px 8px;
          background: var(--cv-bg);
          border: 1px solid var(--cv-border);
          border-radius: 12px;
          font-size: 0.8rem;
          font-family: var(--cv-font-mono);
        }
        .cv-vd-entries {
          padding: 12px;
          padding-left: 32px;
          border-top: 1px solid var(--cv-border);
          display: flex;
          flex-direction: column;
          gap: 8px;
        }
        .cv-vd-entry {
          display: flex;
          gap: 8px;
          align-items: center;
        }
        .cv-vd-entry input.value-input {
          width: 80px;
          flex-shrink: 0;
        }
        .cv-vd-entry input.desc-input {
          flex: 1;
        }
        .cv-vd-add-entry {
          display: flex;
          gap: 8px;
          align-items: center;
          margin-top: 8px;
          padding-top: 8px;
          border-top: 1px dashed var(--cv-border);
        }
        .cv-vd-add-entry input.value-input {
          width: 80px;
          flex-shrink: 0;
        }
        .cv-vd-add-entry input.desc-input {
          flex: 1;
        }
        .cv-btn-icon {
          background: none;
          border: none;
          color: var(--cv-text-muted);
          cursor: pointer;
          font-size: 1.1rem;
          padding: 2px 6px;
          line-height: 1;
          border-radius: 4px;
        }
        .cv-btn-icon:hover {
          background: var(--cv-danger);
          color: white;
        }
        .cv-vd-footer {
          margin-top: 16px;
          padding-top: 16px;
          border-top: 1px solid var(--cv-border);
        }
        .cv-vd-create-inline {
          display: flex;
          gap: 8px;
          align-items: center;
        }
        .cv-vd-create-inline select {
          flex: 1;
          max-width: 300px;
        }
      </style>

      ${this.valueDescriptions.length>0?`
        <div class="cv-vd-list">${e}</div>
      `:`
        <p style="color: var(--cv-text-muted); font-style: italic; margin: 0 0 12px 0;">
          No value descriptions defined.
        </p>
      `}

      <div class="cv-vd-footer">
        ${this.showingCreateForm?`
          <div class="cv-vd-create-inline">
            <select class="cv-select" id="signal-select">
              <option value="">Select signal...</option>
              ${this.availableSignals.map((t,s)=>`
                <option value="${s}">${g(t.signalName)} (${g(t.messageName)})</option>
              `).join("")}
            </select>
            <button class="cv-btn primary small" id="confirm-create-btn">Create</button>
            <button class="cv-btn small" id="cancel-create-btn">Cancel</button>
          </div>
        `:`
          <button class="cv-btn primary" id="new-btn" ${this.availableSignals.length===0?'disabled title="All signals already have value descriptions"':""}>
            + New Value Description
          </button>
        `}
      </div>
    `,this.setupEventListeners()}renderChips(e){if(e.descriptions.length===0)return'<div class="cv-vd-chips"><em style="color: var(--cv-text-muted); font-size: 0.85rem;">Click to add values</em></div>';const t=5,s=e.descriptions.slice(0,t).map(n=>`<span class="cv-vd-chip">${n.value} = ${g(n.description)}</span>`).join(""),a=e.descriptions.length>t?`<span class="cv-vd-chip">+${e.descriptions.length-t} more</span>`:"";return`<div class="cv-vd-chips">${s}${a}</div>`}renderEntries(e,t){return`
      <div class="cv-vd-entries">
        ${e.descriptions.map((a,n)=>`
      <div class="cv-vd-entry">
        <input type="number" class="cv-input value-input" data-key="${t}" data-idx="${n}" value="${a.value}">
        <span>=</span>
        <input type="text" class="cv-input desc-input" data-key="${t}" data-idx="${n}" value="${g(a.description)}">
        <button class="cv-btn-icon remove-btn" data-key="${t}" data-idx="${n}" title="Remove">&times;</button>
      </div>
    `).join("")||'<em style="color: var(--cv-text-muted);">No values yet</em>'}
        <div class="cv-vd-add-entry">
          <input type="number" class="cv-input value-input" id="add-value-${t}" placeholder="Value">
          <span>=</span>
          <input type="text" class="cv-input desc-input" id="add-desc-${t}" placeholder="Description">
          <button class="cv-btn small primary add-btn" data-key="${t}">Add</button>
        </div>
      </div>
    `}getAvailableSignals(){const e=[];for(const t of this.messages)for(const s of t.signals)this.valueDescriptions.some(n=>n.message_id===t.id&&n.signal_name===s.name)||e.push({messageId:t.id,signalName:s.name,messageName:t.name});return e}setupEventListeners(){this.shadowRoot&&(this.shadowRoot.querySelectorAll("[data-toggle]").forEach(e=>{e.addEventListener("click",t=>{if(t.target.closest(".cv-btn-icon"))return;const s=e.dataset.toggle;this.expandedKey=this.expandedKey===s?null:s,this.render()})}),this.shadowRoot.querySelectorAll(".delete-btn").forEach(e=>{e.addEventListener("click",t=>{t.stopPropagation();const s=e.dataset.delete,[a,n]=this.parseKey(s);this.valueDescriptions=this.valueDescriptions.filter(r=>!(r.message_id===a&&r.signal_name===n)),this.expandedKey===s&&(this.expandedKey=null),this.notifyChange(),this.render()})}),this.shadowRoot.getElementById("new-btn")?.addEventListener("click",()=>{this.showingCreateForm=!0,this.render()}),this.shadowRoot.getElementById("cancel-create-btn")?.addEventListener("click",()=>{this.showingCreateForm=!1,this.render()}),this.shadowRoot.getElementById("confirm-create-btn")?.addEventListener("click",()=>{const e=this.shadowRoot.getElementById("signal-select");if(e.value===""){alert("Please select a signal");return}const t=this.availableSignals[parseInt(e.value)];if(!t)return;const s={message_id:t.messageId,signal_name:t.signalName,descriptions:[]};this.valueDescriptions.push(s),this.expandedKey=this.getKey(s),this.showingCreateForm=!1,this.notifyChange(),this.render()}),this.shadowRoot.querySelectorAll(".add-btn").forEach(e=>{e.addEventListener("click",()=>{const t=e.dataset.key,s=this.shadowRoot.getElementById(`add-value-${t}`),a=this.shadowRoot.getElementById(`add-desc-${t}`);if(!s.value||!a.value.trim())return;const[n,r]=this.parseKey(t),o=this.valueDescriptions.find(l=>l.message_id===n&&l.signal_name===r);o&&(o.descriptions.push({value:parseInt(s.value),description:a.value.trim()}),this.notifyChange(),this.render())})}),this.shadowRoot.querySelectorAll(".remove-btn").forEach(e=>{e.addEventListener("click",()=>{const t=e.dataset.key,s=parseInt(e.dataset.idx),[a,n]=this.parseKey(t),r=this.valueDescriptions.find(o=>o.message_id===a&&o.signal_name===n);r&&(r.descriptions.splice(s,1),this.notifyChange(),this.render())})}),this.shadowRoot.querySelectorAll(".cv-vd-entry input").forEach(e=>{e.addEventListener("change",()=>{const t=e.dataset.key,s=parseInt(e.dataset.idx),[a,n]=this.parseKey(t),r=this.valueDescriptions.find(o=>o.message_id===a&&o.signal_name===n);r&&r.descriptions[s]&&(e.classList.contains("value-input")?r.descriptions[s].value=parseInt(e.value):r.descriptions[s].description=e.value.trim(),this.notifyChange())})}))}parseKey(e){const t=e.indexOf(":");return[parseInt(e.substring(0,t)),e.substring(t+1)]}notifyChange(){this.dispatchEvent(w("value-descriptions-change",{valueDescriptions:this.valueDescriptions}))}}customElements.define("cv-value-descriptions-editor",Zt);class Gt extends HTMLElement{definitions=[];defaults=[];values=[];messages=[];nodes=[];activeSubtab="definitions";addingDef=!1;constructor(){super(),this.attachShadow({mode:"open"})}connectedCallback(){this.render()}setData(e){this.definitions=e.definitions.map(t=>({...t,value_type:{...t.value_type}})),this.defaults=e.defaults.map(t=>({...t})),this.values=e.values.map(t=>({...t,target:{...t.target}})),this.render()}setMessages(e){this.messages=e}setNodes(e){this.nodes=e}getData(){return{definitions:this.definitions,defaults:this.defaults,values:this.values}}render(){this.shadowRoot&&(this.shadowRoot.innerHTML=`
      <style>${L}
        :host { display: block; }
        .cv-attr-tabs {
          display: flex;
          gap: 4px;
          margin-bottom: 16px;
          border-bottom: 1px solid var(--cv-border);
          padding-bottom: 8px;
        }
        .cv-attr-tab {
          padding: 6px 12px;
          background: none;
          border: 1px solid transparent;
          border-radius: var(--cv-radius) var(--cv-radius) 0 0;
          cursor: pointer;
          color: var(--cv-text-muted);
          font-size: 0.9rem;
        }
        .cv-attr-tab:hover {
          color: var(--cv-text);
        }
        .cv-attr-tab.active {
          color: var(--cv-accent);
          border-color: var(--cv-border);
          border-bottom-color: var(--cv-bg);
          background: var(--cv-bg);
        }
        .cv-attr-list {
          display: flex;
          flex-direction: column;
          gap: 8px;
          margin-bottom: 16px;
        }
        .cv-attr-item {
          padding: 10px 12px;
          background: var(--cv-bg-elevated);
          border: 1px solid var(--cv-border);
          border-radius: var(--cv-radius);
          cursor: pointer;
          transition: border-color 0.15s;
        }
        .cv-attr-item:hover {
          border-color: var(--cv-accent);
        }
        .cv-attr-item.editing {
          cursor: default;
          border-color: var(--cv-accent);
        }
        .cv-attr-header {
          display: flex;
          align-items: center;
          gap: 8px;
        }
        .cv-attr-name {
          font-weight: 600;
          color: var(--cv-text);
        }
        .cv-attr-type {
          font-size: 0.8rem;
          padding: 2px 6px;
          background: var(--cv-bg);
          border-radius: 4px;
          color: var(--cv-text-muted);
        }
        .cv-attr-meta {
          font-size: 0.85rem;
          color: var(--cv-text-muted);
          margin-top: 4px;
        }
        .cv-attr-form {
          display: grid;
          gap: 12px;
          margin-top: 12px;
        }
        .cv-attr-form-row {
          display: flex;
          gap: 12px;
          flex-wrap: wrap;
        }
        .cv-attr-form-row .cv-form-group {
          flex: 1;
          min-width: 120px;
        }
        .cv-btn-icon {
          background: none;
          border: none;
          color: var(--cv-text-muted);
          cursor: pointer;
          font-size: 1.2rem;
          padding: 2px 6px;
          line-height: 1;
          margin-left: auto;
        }
        .cv-btn-icon:hover {
          color: var(--cv-danger);
        }
      </style>

      <div class="cv-attr-tabs">
        <button class="cv-attr-tab ${this.activeSubtab==="definitions"?"active":""}" data-tab="definitions">
          Definitions (${this.definitions.length})
        </button>
        <button class="cv-attr-tab ${this.activeSubtab==="defaults"?"active":""}" data-tab="defaults">
          Defaults (${this.defaults.length})
        </button>
        <button class="cv-attr-tab ${this.activeSubtab==="values"?"active":""}" data-tab="values">
          Values (${this.values.length})
        </button>
      </div>

      ${this.activeSubtab==="definitions"?this.renderDefinitions():""}
      ${this.activeSubtab==="defaults"?this.renderDefaults():""}
      ${this.activeSubtab==="values"?this.renderValues():""}
    `,this.setupEventListeners())}renderDefinitions(){const e=this.definitions.map(t=>{const s=this.getTypeLabel(t.value_type);return`
        <div class="cv-attr-item" data-def="${g(t.name)}">
          <div class="cv-attr-header">
            <span class="cv-attr-name">${g(t.name)}</span>
            <span class="cv-attr-type">${g(t.object_type)}</span>
            <span class="cv-attr-type">${g(s)}</span>
            <button class="cv-btn-icon" data-delete-def="${g(t.name)}">&times;</button>
          </div>
        </div>
      `}).join("");return`
      ${this.definitions.length===0?`
        <p class="cv-empty-message" style="font-style: italic; margin-bottom: 12px;">
          No attribute definitions. Attribute definitions specify what attributes can be used.
        </p>
      `:`
        <div class="cv-attr-list">${e}</div>
      `}

      ${this.addingDef?this.renderAddDefForm():`
        <button class="cv-btn primary" id="add-def-btn">Add Definition</button>
      `}
    `}renderAddDefForm(){return`
      <div class="cv-attr-item editing">
        <div class="cv-attr-form">
          <div class="cv-attr-form-row">
            <div class="cv-form-group">
              <label class="cv-label">Name</label>
              <input type="text" class="cv-input" id="new-def-name" placeholder="e.g., GenMsgCycleTime">
            </div>
            <div class="cv-form-group">
              <label class="cv-label">Object Type</label>
              <select class="cv-select" id="new-def-object">
                <option value="network">Network</option>
                <option value="node">Node</option>
                <option value="message">Message</option>
                <option value="signal">Signal</option>
              </select>
            </div>
          </div>
          <div class="cv-attr-form-row">
            <div class="cv-form-group">
              <label class="cv-label">Value Type</label>
              <select class="cv-select" id="new-def-type">
                <option value="int">Integer</option>
                <option value="float">Float</option>
                <option value="string">String</option>
                <option value="enum">Enum</option>
                <option value="hex">Hex</option>
              </select>
            </div>
            <div class="cv-form-group" id="new-def-min-group">
              <label class="cv-label">Min</label>
              <input type="number" class="cv-input" id="new-def-min" value="0">
            </div>
            <div class="cv-form-group" id="new-def-max-group">
              <label class="cv-label">Max</label>
              <input type="number" class="cv-input" id="new-def-max" value="65535">
            </div>
          </div>
          <div class="cv-form-group" id="new-def-enum-group" style="display: none;">
            <label class="cv-label">Enum Values (comma-separated)</label>
            <input type="text" class="cv-input" id="new-def-enum" placeholder="e.g., Off,On,Auto">
          </div>
          <div style="display: flex; gap: 8px;">
            <button class="cv-btn small success" id="save-def-btn">Create</button>
            <button class="cv-btn small" id="cancel-def-btn">Cancel</button>
          </div>
        </div>
      </div>
    `}renderDefaults(){const e=this.defaults.map(s=>`
        <div class="cv-attr-item" data-default="${g(s.name)}">
          <div class="cv-attr-header">
            <span class="cv-attr-name">${g(s.name)}</span>
            <span class="cv-attr-type">= ${g(String(s.value))}</span>
            <button class="cv-btn-icon" data-delete-default="${g(s.name)}">&times;</button>
          </div>
        </div>
      `).join(""),t=this.definitions.filter(s=>!this.defaults.some(a=>a.name===s.name));return`
      ${this.defaults.length===0?`
        <p class="cv-empty-message" style="font-style: italic; margin-bottom: 12px;">
          No attribute defaults. Defaults are used when an attribute is not explicitly set.
        </p>
      `:`
        <div class="cv-attr-list">${e}</div>
      `}

      <div style="display: flex; gap: 8px; align-items: end; flex-wrap: wrap;">
        <div class="cv-form-group" style="flex: 1; min-width: 150px;">
          <label class="cv-label">Attribute</label>
          <select class="cv-select" id="add-default-attr">
            <option value="">Select attribute...</option>
            ${t.map(s=>`<option value="${g(s.name)}">${g(s.name)}</option>`).join("")}
          </select>
        </div>
        <div class="cv-form-group" style="flex: 1; min-width: 100px;">
          <label class="cv-label">Default Value</label>
          <input type="text" class="cv-input" id="add-default-value" placeholder="Value">
        </div>
        <button class="cv-btn primary" id="add-default-btn">Add Default</button>
      </div>
    `}renderValues(){const e=this.values.map((t,s)=>{const a=this.getTargetLabel(t.target);return`
        <div class="cv-attr-item" data-value-index="${s}">
          <div class="cv-attr-header">
            <span class="cv-attr-name">${g(t.name)}</span>
            <span class="cv-attr-type">${g(a)}</span>
            <span class="cv-attr-type">= ${g(String(t.value))}</span>
            <button class="cv-btn-icon" data-delete-value="${s}">&times;</button>
          </div>
        </div>
      `}).join("");return`
      ${this.values.length===0?`
        <p class="cv-empty-message" style="font-style: italic; margin-bottom: 12px;">
          No attribute values assigned. Values assign attributes to specific objects.
        </p>
      `:`
        <div class="cv-attr-list">${e}</div>
      `}

      <div style="display: flex; gap: 8px; align-items: end; flex-wrap: wrap;">
        <div class="cv-form-group" style="flex: 1; min-width: 120px;">
          <label class="cv-label">Attribute</label>
          <select class="cv-select" id="add-value-attr">
            <option value="">Select...</option>
            ${this.definitions.map(t=>`<option value="${g(t.name)}">${g(t.name)} (${t.object_type})</option>`).join("")}
          </select>
        </div>
        <div class="cv-form-group" style="flex: 1; min-width: 120px;" id="add-value-target-group">
          <label class="cv-label">Target</label>
          <select class="cv-select" id="add-value-target">
            <option value="">Select attribute first</option>
          </select>
        </div>
        <div class="cv-form-group" style="flex: 1; min-width: 80px;">
          <label class="cv-label">Value</label>
          <input type="text" class="cv-input" id="add-value-value" placeholder="Value">
        </div>
        <button class="cv-btn primary" id="add-value-btn">Add Value</button>
      </div>
    `}getTypeLabel(e){switch(e.type){case"int":return`int [${e.min}, ${e.max}]`;case"float":return`float [${e.min}, ${e.max}]`;case"hex":return`hex [${e.min}, ${e.max}]`;case"string":return"string";case"enum":return`enum (${e.values.join(", ")})`;default:return"unknown"}}getTargetLabel(e){switch(e.type){case"network":return"Network";case"node":return`Node: ${e.node_name}`;case"message":return`Message: ${P(e.message_id,!1)}`;case"signal":{const s=this.messages.find(a=>a.id===e.message_id)?.name||P(e.message_id,!1);return`Signal: ${e.signal_name} (${s})`}default:return"Unknown"}}setupEventListeners(){if(!this.shadowRoot)return;this.shadowRoot.querySelectorAll(".cv-attr-tab").forEach(s=>{s.addEventListener("click",()=>{this.activeSubtab=s.dataset.tab,this.render()})}),this.shadowRoot.getElementById("add-def-btn")?.addEventListener("click",()=>{this.addingDef=!0,this.render()}),this.shadowRoot.getElementById("cancel-def-btn")?.addEventListener("click",()=>{this.addingDef=!1,this.render()}),this.shadowRoot.getElementById("save-def-btn")?.addEventListener("click",()=>{this.handleSaveDefinition()}),this.shadowRoot.querySelectorAll("[data-delete-def]").forEach(s=>{s.addEventListener("click",a=>{a.stopPropagation();const n=s.dataset.deleteDef;n&&(this.definitions=this.definitions.filter(r=>r.name!==n),this.defaults=this.defaults.filter(r=>r.name!==n),this.values=this.values.filter(r=>r.name!==n),this.notifyChange(),this.render())})});const e=this.shadowRoot.getElementById("new-def-type");e?.addEventListener("change",()=>{const s=this.shadowRoot.getElementById("new-def-min-group"),a=this.shadowRoot.getElementById("new-def-max-group"),n=this.shadowRoot.getElementById("new-def-enum-group");e.value==="enum"?(s.style.display="none",a.style.display="none",n.style.display="block"):e.value==="string"?(s.style.display="none",a.style.display="none",n.style.display="none"):(s.style.display="block",a.style.display="block",n.style.display="none")}),this.shadowRoot.getElementById("add-default-btn")?.addEventListener("click",()=>{this.handleAddDefault()}),this.shadowRoot.querySelectorAll("[data-delete-default]").forEach(s=>{s.addEventListener("click",a=>{a.stopPropagation();const n=s.dataset.deleteDefault;n&&(this.defaults=this.defaults.filter(r=>r.name!==n),this.notifyChange(),this.render())})}),this.shadowRoot.getElementById("add-value-btn")?.addEventListener("click",()=>{this.handleAddValue()}),this.shadowRoot.querySelectorAll("[data-delete-value]").forEach(s=>{s.addEventListener("click",a=>{a.stopPropagation();const n=parseInt(s.dataset.deleteValue||"0");this.values.splice(n,1),this.notifyChange(),this.render()})});const t=this.shadowRoot.getElementById("add-value-attr");t?.addEventListener("change",()=>{this.updateTargetOptions(t.value)})}updateTargetOptions(e){const t=this.shadowRoot.getElementById("add-value-target");if(!t)return;const s=this.definitions.find(n=>n.name===e);if(!s){t.innerHTML='<option value="">Select attribute first</option>';return}let a="";switch(s.object_type){case"network":a='<option value="network">Network</option>';break;case"node":a=this.nodes.map(n=>`<option value="node:${g(n.name)}">${g(n.name)}</option>`).join("");break;case"message":a=this.messages.map(n=>`<option value="message:${n.id}">${g(n.name)} (${P(n.id,n.is_extended)})</option>`).join("");break;case"signal":for(const n of this.messages)for(const r of n.signals)a+=`<option value="signal:${n.id}:${g(r.name)}">${g(r.name)} (${g(n.name)})</option>`;break}t.innerHTML=a||'<option value="">No targets available</option>'}handleSaveDefinition(){const e=this.shadowRoot.getElementById("new-def-name").value.trim(),t=this.shadowRoot.getElementById("new-def-object").value,s=this.shadowRoot.getElementById("new-def-type").value;if(!e){alert("Name is required");return}if(this.definitions.some(n=>n.name===e)){alert("An attribute with this name already exists");return}let a;if(s==="int"||s==="hex"){const n=parseInt(this.shadowRoot.getElementById("new-def-min").value)||0,r=parseInt(this.shadowRoot.getElementById("new-def-max").value)||65535;a={type:s,min:n,max:r}}else if(s==="float"){const n=parseFloat(this.shadowRoot.getElementById("new-def-min").value)||0,r=parseFloat(this.shadowRoot.getElementById("new-def-max").value)||1e3;a={type:"float",min:n,max:r}}else if(s==="enum"){const r=this.shadowRoot.getElementById("new-def-enum").value.split(",").map(o=>o.trim()).filter(o=>o);if(r.length===0){alert("Enum requires at least one value");return}a={type:"enum",values:r}}else a={type:"string"};this.definitions.push({name:e,object_type:t,value_type:a}),this.addingDef=!1,this.notifyChange(),this.render()}handleAddDefault(){const e=this.shadowRoot.getElementById("add-default-attr"),t=this.shadowRoot.getElementById("add-default-value");if(!e.value||!t.value){alert("Please select an attribute and enter a value");return}const s=this.definitions.find(n=>n.name===e.value);let a=t.value;s&&(s.value_type.type==="int"||s.value_type.type==="hex")?a=parseInt(a)||0:s&&s.value_type.type==="float"&&(a=parseFloat(a)||0),this.defaults.push({name:e.value,value:a}),this.notifyChange(),this.render()}handleAddValue(){const e=this.shadowRoot.getElementById("add-value-attr"),t=this.shadowRoot.getElementById("add-value-target"),s=this.shadowRoot.getElementById("add-value-value");if(!e.value||!t.value||!s.value){alert("Please fill in all fields");return}let a;const n=t.value;if(n==="network")a={type:"network"};else if(n.startsWith("node:"))a={type:"node",node_name:n.substring(5)};else if(n.startsWith("message:"))a={type:"message",message_id:parseInt(n.substring(8))};else if(n.startsWith("signal:")){const l=n.substring(7).split(":");a={type:"signal",message_id:parseInt(l[0]),signal_name:l[1]}}else{alert("Invalid target");return}const r=this.definitions.find(l=>l.name===e.value);let o=s.value;r&&(r.value_type.type==="int"||r.value_type.type==="hex")?o=parseInt(o)||0:r&&r.value_type.type==="float"&&(o=parseFloat(o)||0),this.values.push({name:e.value,target:a,value:o}),this.notifyChange(),this.render()}notifyChange(){this.dispatchEvent(w("attributes-change",{definitions:this.definitions,defaults:this.defaults,values:this.values}))}}customElements.define("cv-attributes-editor",Gt);function le(){return{async:!1,breaks:!1,extensions:null,gfm:!0,hooks:null,pedantic:!1,renderer:null,silent:!1,tokenizer:null,walkTokens:null}}var z=le();function Ne(i){z=i}var X={exec:()=>null};function b(i,e=""){let t=typeof i=="string"?i:i.source,s={replace:(a,n)=>{let r=typeof n=="string"?n:n.source;return r=r.replace(E.caret,"$1"),t=t.replace(a,r),s},getRegex:()=>new RegExp(t,e)};return s}var Kt=(()=>{try{return!!new RegExp("(?<=1)(?<!1)")}catch{return!1}})(),E={codeRemoveIndent:/^(?: {1,4}| {0,3}\t)/gm,outputLinkReplace:/\\([\[\]])/g,indentCodeCompensation:/^(\s+)(?:```)/,beginningSpace:/^\s+/,endingHash:/#$/,startingSpaceChar:/^ /,endingSpaceChar:/ $/,nonSpaceChar:/[^ ]/,newLineCharGlobal:/\n/g,tabCharGlobal:/\t/g,multipleSpaceGlobal:/\s+/g,blankLine:/^[ \t]*$/,doubleBlankLine:/\n[ \t]*\n[ \t]*$/,blockquoteStart:/^ {0,3}>/,blockquoteSetextReplace:/\n {0,3}((?:=+|-+) *)(?=\n|$)/g,blockquoteSetextReplace2:/^ {0,3}>[ \t]?/gm,listReplaceTabs:/^\t+/,listReplaceNesting:/^ {1,4}(?=( {4})*[^ ])/g,listIsTask:/^\[[ xX]\] +\S/,listReplaceTask:/^\[[ xX]\] +/,listTaskCheckbox:/\[[ xX]\]/,anyLine:/\n.*\n/,hrefBrackets:/^<(.*)>$/,tableDelimiter:/[:|]/,tableAlignChars:/^\||\| *$/g,tableRowBlankLine:/\n[ \t]*$/,tableAlignRight:/^ *-+: *$/,tableAlignCenter:/^ *:-+: *$/,tableAlignLeft:/^ *:-+ *$/,startATag:/^<a /i,endATag:/^<\/a>/i,startPreScriptTag:/^<(pre|code|kbd|script)(\s|>)/i,endPreScriptTag:/^<\/(pre|code|kbd|script)(\s|>)/i,startAngleBracket:/^</,endAngleBracket:/>$/,pedanticHrefTitle:/^([^'"]*[^\s])\s+(['"])(.*)\2/,unicodeAlphaNumeric:/[\p{L}\p{N}]/u,escapeTest:/[&<>"']/,escapeReplace:/[&<>"']/g,escapeTestNoEncode:/[<>"']|&(?!(#\d{1,7}|#[Xx][a-fA-F0-9]{1,6}|\w+);)/,escapeReplaceNoEncode:/[<>"']|&(?!(#\d{1,7}|#[Xx][a-fA-F0-9]{1,6}|\w+);)/g,unescapeTest:/&(#(?:\d+)|(?:#x[0-9A-Fa-f]+)|(?:\w+));?/ig,caret:/(^|[^\[])\^/g,percentDecode:/%25/g,findPipe:/\|/g,splitPipe:/ \|/,slashPipe:/\\\|/g,carriageReturn:/\r\n|\r/g,spaceLine:/^ +$/gm,notSpaceStart:/^\S*/,endingNewline:/\n$/,listItemRegex:i=>new RegExp(`^( {0,3}${i})((?:[	 ][^\\n]*)?(?:\\n|$))`),nextBulletRegex:i=>new RegExp(`^ {0,${Math.min(3,i-1)}}(?:[*+-]|\\d{1,9}[.)])((?:[ 	][^\\n]*)?(?:\\n|$))`),hrRegex:i=>new RegExp(`^ {0,${Math.min(3,i-1)}}((?:- *){3,}|(?:_ *){3,}|(?:\\* *){3,})(?:\\n+|$)`),fencesBeginRegex:i=>new RegExp(`^ {0,${Math.min(3,i-1)}}(?:\`\`\`|~~~)`),headingBeginRegex:i=>new RegExp(`^ {0,${Math.min(3,i-1)}}#`),htmlBeginRegex:i=>new RegExp(`^ {0,${Math.min(3,i-1)}}<(?:[a-z].*>|!--)`,"i")},Qt=/^(?:[ \t]*(?:\n|$))+/,Wt=/^((?: {4}| {0,3}\t)[^\n]+(?:\n(?:[ \t]*(?:\n|$))*)?)+/,Yt=/^ {0,3}(`{3,}(?=[^`\n]*(?:\n|$))|~{3,})([^\n]*)(?:\n|$)(?:|([\s\S]*?)(?:\n|$))(?: {0,3}\1[~`]* *(?=\n|$)|$)/,Z=/^ {0,3}((?:-[\t ]*){3,}|(?:_[ \t]*){3,}|(?:\*[ \t]*){3,})(?:\n+|$)/,Jt=/^ {0,3}(#{1,6})(?=\s|$)(.*)(?:\n+|$)/,oe=/(?:[*+-]|\d{1,9}[.)])/,Pe=/^(?!bull |blockCode|fences|blockquote|heading|html|table)((?:.|\n(?!\s*?\n|bull |blockCode|fences|blockquote|heading|html|table))+?)\n {0,3}(=+|-+) *(?:\n+|$)/,je=b(Pe).replace(/bull/g,oe).replace(/blockCode/g,/(?: {4}| {0,3}\t)/).replace(/fences/g,/ {0,3}(?:`{3,}|~{3,})/).replace(/blockquote/g,/ {0,3}>/).replace(/heading/g,/ {0,3}#{1,6}/).replace(/html/g,/ {0,3}<[^\n>]+>\n/).replace(/\|table/g,"").getRegex(),es=b(Pe).replace(/bull/g,oe).replace(/blockCode/g,/(?: {4}| {0,3}\t)/).replace(/fences/g,/ {0,3}(?:`{3,}|~{3,})/).replace(/blockquote/g,/ {0,3}>/).replace(/heading/g,/ {0,3}#{1,6}/).replace(/html/g,/ {0,3}<[^\n>]+>\n/).replace(/table/g,/ {0,3}\|?(?:[:\- ]*\|)+[\:\- ]*\n/).getRegex(),ce=/^([^\n]+(?:\n(?!hr|heading|lheading|blockquote|fences|list|html|table| +\n)[^\n]+)*)/,ts=/^[^\n]+/,de=/(?!\s*\])(?:\\[\s\S]|[^\[\]\\])+/,ss=b(/^ {0,3}\[(label)\]: *(?:\n[ \t]*)?([^<\s][^\s]*|<.*?>)(?:(?: +(?:\n[ \t]*)?| *\n[ \t]*)(title))? *(?:\n+|$)/).replace("label",de).replace("title",/(?:"(?:\\"?|[^"\\])*"|'[^'\n]*(?:\n[^'\n]+)*\n?'|\([^()]*\))/).getRegex(),as=b(/^( {0,3}bull)([ \t][^\n]+?)?(?:\n|$)/).replace(/bull/g,oe).getRegex(),ee="address|article|aside|base|basefont|blockquote|body|caption|center|col|colgroup|dd|details|dialog|dir|div|dl|dt|fieldset|figcaption|figure|footer|form|frame|frameset|h[1-6]|head|header|hr|html|iframe|legend|li|link|main|menu|menuitem|meta|nav|noframes|ol|optgroup|option|p|param|search|section|summary|table|tbody|td|tfoot|th|thead|title|tr|track|ul",he=/<!--(?:-?>|[\s\S]*?(?:-->|$))/,is=b("^ {0,3}(?:<(script|pre|style|textarea)[\\s>][\\s\\S]*?(?:</\\1>[^\\n]*\\n+|$)|comment[^\\n]*(\\n+|$)|<\\?[\\s\\S]*?(?:\\?>\\n*|$)|<![A-Z][\\s\\S]*?(?:>\\n*|$)|<!\\[CDATA\\[[\\s\\S]*?(?:\\]\\]>\\n*|$)|</?(tag)(?: +|\\n|/?>)[\\s\\S]*?(?:(?:\\n[ 	]*)+\\n|$)|<(?!script|pre|style|textarea)([a-z][\\w-]*)(?:attribute)*? */?>(?=[ \\t]*(?:\\n|$))[\\s\\S]*?(?:(?:\\n[ 	]*)+\\n|$)|</(?!script|pre|style|textarea)[a-z][\\w-]*\\s*>(?=[ \\t]*(?:\\n|$))[\\s\\S]*?(?:(?:\\n[ 	]*)+\\n|$))","i").replace("comment",he).replace("tag",ee).replace("attribute",/ +[a-zA-Z:_][\w.:-]*(?: *= *"[^"\n]*"| *= *'[^'\n]*'| *= *[^\s"'=<>`]+)?/).getRegex(),Ve=b(ce).replace("hr",Z).replace("heading"," {0,3}#{1,6}(?:\\s|$)").replace("|lheading","").replace("|table","").replace("blockquote"," {0,3}>").replace("fences"," {0,3}(?:`{3,}(?=[^`\\n]*\\n)|~{3,})[^\\n]*\\n").replace("list"," {0,3}(?:[*+-]|1[.)]) ").replace("html","</?(?:tag)(?: +|\\n|/?>)|<(?:script|pre|style|textarea|!--)").replace("tag",ee).getRegex(),ns=b(/^( {0,3}> ?(paragraph|[^\n]*)(?:\n|$))+/).replace("paragraph",Ve).getRegex(),pe={blockquote:ns,code:Wt,def:ss,fences:Yt,heading:Jt,hr:Z,html:is,lheading:je,list:as,newline:Qt,paragraph:Ve,table:X,text:ts},Me=b("^ *([^\\n ].*)\\n {0,3}((?:\\| *)?:?-+:? *(?:\\| *:?-+:? *)*(?:\\| *)?)(?:\\n((?:(?! *\\n|hr|heading|blockquote|code|fences|list|html).*(?:\\n|$))*)\\n*|$)").replace("hr",Z).replace("heading"," {0,3}#{1,6}(?:\\s|$)").replace("blockquote"," {0,3}>").replace("code","(?: {4}| {0,3}	)[^\\n]").replace("fences"," {0,3}(?:`{3,}(?=[^`\\n]*\\n)|~{3,})[^\\n]*\\n").replace("list"," {0,3}(?:[*+-]|1[.)]) ").replace("html","</?(?:tag)(?: +|\\n|/?>)|<(?:script|pre|style|textarea|!--)").replace("tag",ee).getRegex(),rs={...pe,lheading:es,table:Me,paragraph:b(ce).replace("hr",Z).replace("heading"," {0,3}#{1,6}(?:\\s|$)").replace("|lheading","").replace("table",Me).replace("blockquote"," {0,3}>").replace("fences"," {0,3}(?:`{3,}(?=[^`\\n]*\\n)|~{3,})[^\\n]*\\n").replace("list"," {0,3}(?:[*+-]|1[.)]) ").replace("html","</?(?:tag)(?: +|\\n|/?>)|<(?:script|pre|style|textarea|!--)").replace("tag",ee).getRegex()},ls={...pe,html:b(`^ *(?:comment *(?:\\n|\\s*$)|<(tag)[\\s\\S]+?</\\1> *(?:\\n{2,}|\\s*$)|<tag(?:"[^"]*"|'[^']*'|\\s[^'"/>\\s]*)*?/?> *(?:\\n{2,}|\\s*$))`).replace("comment",he).replace(/tag/g,"(?!(?:a|em|strong|small|s|cite|q|dfn|abbr|data|time|code|var|samp|kbd|sub|sup|i|b|u|mark|ruby|rt|rp|bdi|bdo|span|br|wbr|ins|del|img)\\b)\\w+(?!:|[^\\w\\s@]*@)\\b").getRegex(),def:/^ *\[([^\]]+)\]: *<?([^\s>]+)>?(?: +(["(][^\n]+[")]))? *(?:\n+|$)/,heading:/^(#{1,6})(.*)(?:\n+|$)/,fences:X,lheading:/^(.+?)\n {0,3}(=+|-+) *(?:\n+|$)/,paragraph:b(ce).replace("hr",Z).replace("heading",` *#{1,6} *[^
]`).replace("lheading",je).replace("|table","").replace("blockquote"," {0,3}>").replace("|fences","").replace("|list","").replace("|html","").replace("|tag","").getRegex()},os=/^\\([!"#$%&'()*+,\-./:;<=>?@\[\]\\^_`{|}~])/,cs=/^(`+)([^`]|[^`][\s\S]*?[^`])\1(?!`)/,He=/^( {2,}|\\)\n(?!\s*$)/,ds=/^(`+|[^`])(?:(?= {2,}\n)|[\s\S]*?(?:(?=[\\<!\[`*_]|\b_|$)|[^ ](?= {2,}\n)))/,te=/[\p{P}\p{S}]/u,ue=/[\s\p{P}\p{S}]/u,Oe=/[^\s\p{P}\p{S}]/u,hs=b(/^((?![*_])punctSpace)/,"u").replace(/punctSpace/g,ue).getRegex(),Ue=/(?!~)[\p{P}\p{S}]/u,ps=/(?!~)[\s\p{P}\p{S}]/u,us=/(?:[^\s\p{P}\p{S}]|~)/u,vs=b(/link|precode-code|html/,"g").replace("link",/\[(?:[^\[\]`]|(?<a>`+)[^`]+\k<a>(?!`))*?\]\((?:\\[\s\S]|[^\\\(\)]|\((?:\\[\s\S]|[^\\\(\)])*\))*\)/).replace("precode-",Kt?"(?<!`)()":"(^^|[^`])").replace("code",/(?<b>`+)[^`]+\k<b>(?!`)/).replace("html",/<(?! )[^<>]*?>/).getRegex(),Xe=/^(?:\*+(?:((?!\*)punct)|[^\s*]))|^_+(?:((?!_)punct)|([^\s_]))/,gs=b(Xe,"u").replace(/punct/g,te).getRegex(),ms=b(Xe,"u").replace(/punct/g,Ue).getRegex(),Ze="^[^_*]*?__[^_*]*?\\*[^_*]*?(?=__)|[^*]+(?=[^*])|(?!\\*)punct(\\*+)(?=[\\s]|$)|notPunctSpace(\\*+)(?!\\*)(?=punctSpace|$)|(?!\\*)punctSpace(\\*+)(?=notPunctSpace)|[\\s](\\*+)(?!\\*)(?=punct)|(?!\\*)punct(\\*+)(?!\\*)(?=punct)|notPunctSpace(\\*+)(?=notPunctSpace)",fs=b(Ze,"gu").replace(/notPunctSpace/g,Oe).replace(/punctSpace/g,ue).replace(/punct/g,te).getRegex(),bs=b(Ze,"gu").replace(/notPunctSpace/g,us).replace(/punctSpace/g,ps).replace(/punct/g,Ue).getRegex(),xs=b("^[^_*]*?\\*\\*[^_*]*?_[^_*]*?(?=\\*\\*)|[^_]+(?=[^_])|(?!_)punct(_+)(?=[\\s]|$)|notPunctSpace(_+)(?!_)(?=punctSpace|$)|(?!_)punctSpace(_+)(?=notPunctSpace)|[\\s](_+)(?!_)(?=punct)|(?!_)punct(_+)(?!_)(?=punct)","gu").replace(/notPunctSpace/g,Oe).replace(/punctSpace/g,ue).replace(/punct/g,te).getRegex(),ys=b(/\\(punct)/,"gu").replace(/punct/g,te).getRegex(),ws=b(/^<(scheme:[^\s\x00-\x1f<>]*|email)>/).replace("scheme",/[a-zA-Z][a-zA-Z0-9+.-]{1,31}/).replace("email",/[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+(@)[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+(?![-_])/).getRegex(),ks=b(he).replace("(?:-->|$)","-->").getRegex(),Ss=b("^comment|^</[a-zA-Z][\\w:-]*\\s*>|^<[a-zA-Z][\\w-]*(?:attribute)*?\\s*/?>|^<\\?[\\s\\S]*?\\?>|^<![a-zA-Z]+\\s[\\s\\S]*?>|^<!\\[CDATA\\[[\\s\\S]*?\\]\\]>").replace("comment",ks).replace("attribute",/\s+[a-zA-Z:_][\w.:-]*(?:\s*=\s*"[^"]*"|\s*=\s*'[^']*'|\s*=\s*[^\s"'=<>`]+)?/).getRegex(),W=/(?:\[(?:\\[\s\S]|[^\[\]\\])*\]|\\[\s\S]|`+[^`]*?`+(?!`)|[^\[\]\\`])*?/,Es=b(/^!?\[(label)\]\(\s*(href)(?:(?:[ \t]*(?:\n[ \t]*)?)(title))?\s*\)/).replace("label",W).replace("href",/<(?:\\.|[^\n<>\\])+>|[^ \t\n\x00-\x1f]*/).replace("title",/"(?:\\"?|[^"\\])*"|'(?:\\'?|[^'\\])*'|\((?:\\\)?|[^)\\])*\)/).getRegex(),Ge=b(/^!?\[(label)\]\[(ref)\]/).replace("label",W).replace("ref",de).getRegex(),Ke=b(/^!?\[(ref)\](?:\[\])?/).replace("ref",de).getRegex(),$s=b("reflink|nolink(?!\\()","g").replace("reflink",Ge).replace("nolink",Ke).getRegex(),Ie=/[hH][tT][tT][pP][sS]?|[fF][tT][pP]/,ve={_backpedal:X,anyPunctuation:ys,autolink:ws,blockSkip:vs,br:He,code:cs,del:X,emStrongLDelim:gs,emStrongRDelimAst:fs,emStrongRDelimUnd:xs,escape:os,link:Es,nolink:Ke,punctuation:hs,reflink:Ge,reflinkSearch:$s,tag:Ss,text:ds,url:X},Cs={...ve,link:b(/^!?\[(label)\]\((.*?)\)/).replace("label",W).getRegex(),reflink:b(/^!?\[(label)\]\s*\[([^\]]*)\]/).replace("label",W).getRegex()},ae={...ve,emStrongRDelimAst:bs,emStrongLDelim:ms,url:b(/^((?:protocol):\/\/|www\.)(?:[a-zA-Z0-9\-]+\.?)+[^\s<]*|^email/).replace("protocol",Ie).replace("email",/[A-Za-z0-9._+-]+(@)[a-zA-Z0-9-_]+(?:\.[a-zA-Z0-9-_]*[a-zA-Z0-9])+(?![-_])/).getRegex(),_backpedal:/(?:[^?!.,:;*_'"~()&]+|\([^)]*\)|&(?![a-zA-Z0-9]+;$)|[?!.,:;*_'"~)]+(?!$))+/,del:/^(~~?)(?=[^\s~])((?:\\[\s\S]|[^\\])*?(?:\\[\s\S]|[^\s~\\]))\1(?=[^~]|$)/,text:b(/^([`~]+|[^`~])(?:(?= {2,}\n)|(?=[a-zA-Z0-9.!#$%&'*+\/=?_`{\|}~-]+@)|[\s\S]*?(?:(?=[\\<!\[`*~_]|\b_|protocol:\/\/|www\.|$)|[^ ](?= {2,}\n)|[^a-zA-Z0-9.!#$%&'*+\/=?_`{\|}~-](?=[a-zA-Z0-9.!#$%&'*+\/=?_`{\|}~-]+@)))/).replace("protocol",Ie).getRegex()},_s={...ae,br:b(He).replace("{2,}","*").getRegex(),text:b(ae.text).replace("\\b_","\\b_| {2,}\\n").replace(/\{2,\}/g,"*").getRegex()},G={normal:pe,gfm:rs,pedantic:ls},V={normal:ve,gfm:ae,breaks:_s,pedantic:Cs},Ms={"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"},Le=i=>Ms[i];function F(i,e){if(e){if(E.escapeTest.test(i))return i.replace(E.escapeReplace,Le)}else if(E.escapeTestNoEncode.test(i))return i.replace(E.escapeReplaceNoEncode,Le);return i}function Te(i){try{i=encodeURI(i).replace(E.percentDecode,"%")}catch{return null}return i}function De(i,e){let t=i.replace(E.findPipe,(n,r,o)=>{let l=!1,p=r;for(;--p>=0&&o[p]==="\\";)l=!l;return l?"|":" |"}),s=t.split(E.splitPipe),a=0;if(s[0].trim()||s.shift(),s.length>0&&!s.at(-1)?.trim()&&s.pop(),e)if(s.length>e)s.splice(e);else for(;s.length<e;)s.push("");for(;a<s.length;a++)s[a]=s[a].trim().replace(E.slashPipe,"|");return s}function H(i,e,t){let s=i.length;if(s===0)return"";let a=0;for(;a<s&&i.charAt(s-a-1)===e;)a++;return i.slice(0,s-a)}function Is(i,e){if(i.indexOf(e[1])===-1)return-1;let t=0;for(let s=0;s<i.length;s++)if(i[s]==="\\")s++;else if(i[s]===e[0])t++;else if(i[s]===e[1]&&(t--,t<0))return s;return t>0?-2:-1}function Fe(i,e,t,s,a){let n=e.href,r=e.title||null,o=i[1].replace(a.other.outputLinkReplace,"$1");s.state.inLink=!0;let l={type:i[0].charAt(0)==="!"?"image":"link",raw:t,href:n,title:r,text:o,tokens:s.inlineTokens(o)};return s.state.inLink=!1,l}function Ls(i,e,t){let s=i.match(t.other.indentCodeCompensation);if(s===null)return e;let a=s[1];return e.split(`
`).map(n=>{let r=n.match(t.other.beginningSpace);if(r===null)return n;let[o]=r;return o.length>=a.length?n.slice(a.length):n}).join(`
`)}var Y=class{options;rules;lexer;constructor(i){this.options=i||z}space(i){let e=this.rules.block.newline.exec(i);if(e&&e[0].length>0)return{type:"space",raw:e[0]}}code(i){let e=this.rules.block.code.exec(i);if(e){let t=e[0].replace(this.rules.other.codeRemoveIndent,"");return{type:"code",raw:e[0],codeBlockStyle:"indented",text:this.options.pedantic?t:H(t,`
`)}}}fences(i){let e=this.rules.block.fences.exec(i);if(e){let t=e[0],s=Ls(t,e[3]||"",this.rules);return{type:"code",raw:t,lang:e[2]?e[2].trim().replace(this.rules.inline.anyPunctuation,"$1"):e[2],text:s}}}heading(i){let e=this.rules.block.heading.exec(i);if(e){let t=e[2].trim();if(this.rules.other.endingHash.test(t)){let s=H(t,"#");(this.options.pedantic||!s||this.rules.other.endingSpaceChar.test(s))&&(t=s.trim())}return{type:"heading",raw:e[0],depth:e[1].length,text:t,tokens:this.lexer.inline(t)}}}hr(i){let e=this.rules.block.hr.exec(i);if(e)return{type:"hr",raw:H(e[0],`
`)}}blockquote(i){let e=this.rules.block.blockquote.exec(i);if(e){let t=H(e[0],`
`).split(`
`),s="",a="",n=[];for(;t.length>0;){let r=!1,o=[],l;for(l=0;l<t.length;l++)if(this.rules.other.blockquoteStart.test(t[l]))o.push(t[l]),r=!0;else if(!r)o.push(t[l]);else break;t=t.slice(l);let p=o.join(`
`),c=p.replace(this.rules.other.blockquoteSetextReplace,`
    $1`).replace(this.rules.other.blockquoteSetextReplace2,"");s=s?`${s}
${p}`:p,a=a?`${a}
${c}`:c;let d=this.lexer.state.top;if(this.lexer.state.top=!0,this.lexer.blockTokens(c,n,!0),this.lexer.state.top=d,t.length===0)break;let h=n.at(-1);if(h?.type==="code")break;if(h?.type==="blockquote"){let u=h,v=u.raw+`
`+t.join(`
`),m=this.blockquote(v);n[n.length-1]=m,s=s.substring(0,s.length-u.raw.length)+m.raw,a=a.substring(0,a.length-u.text.length)+m.text;break}else if(h?.type==="list"){let u=h,v=u.raw+`
`+t.join(`
`),m=this.list(v);n[n.length-1]=m,s=s.substring(0,s.length-h.raw.length)+m.raw,a=a.substring(0,a.length-u.raw.length)+m.raw,t=v.substring(n.at(-1).raw.length).split(`
`);continue}}return{type:"blockquote",raw:s,tokens:n,text:a}}}list(i){let e=this.rules.block.list.exec(i);if(e){let t=e[1].trim(),s=t.length>1,a={type:"list",raw:"",ordered:s,start:s?+t.slice(0,-1):"",loose:!1,items:[]};t=s?`\\d{1,9}\\${t.slice(-1)}`:`\\${t}`,this.options.pedantic&&(t=s?t:"[*+-]");let n=this.rules.other.listItemRegex(t),r=!1;for(;i;){let l=!1,p="",c="";if(!(e=n.exec(i))||this.rules.block.hr.test(i))break;p=e[0],i=i.substring(p.length);let d=e[2].split(`
`,1)[0].replace(this.rules.other.listReplaceTabs,m=>" ".repeat(3*m.length)),h=i.split(`
`,1)[0],u=!d.trim(),v=0;if(this.options.pedantic?(v=2,c=d.trimStart()):u?v=e[1].length+1:(v=e[2].search(this.rules.other.nonSpaceChar),v=v>4?1:v,c=d.slice(v),v+=e[1].length),u&&this.rules.other.blankLine.test(h)&&(p+=h+`
`,i=i.substring(h.length+1),l=!0),!l){let m=this.rules.other.nextBulletRegex(v),y=this.rules.other.hrRegex(v),S=this.rules.other.fencesBeginRegex(v),T=this.rules.other.headingBeginRegex(v),A=this.rules.other.htmlBeginRegex(v);for(;i;){let C=i.split(`
`,1)[0],D;if(h=C,this.options.pedantic?(h=h.replace(this.rules.other.listReplaceNesting,"  "),D=h):D=h.replace(this.rules.other.tabCharGlobal,"    "),S.test(h)||T.test(h)||A.test(h)||m.test(h)||y.test(h))break;if(D.search(this.rules.other.nonSpaceChar)>=v||!h.trim())c+=`
`+D.slice(v);else{if(u||d.replace(this.rules.other.tabCharGlobal,"    ").search(this.rules.other.nonSpaceChar)>=4||S.test(d)||T.test(d)||y.test(d))break;c+=`
`+h}!u&&!h.trim()&&(u=!0),p+=C+`
`,i=i.substring(C.length+1),d=D.slice(v)}}a.loose||(r?a.loose=!0:this.rules.other.doubleBlankLine.test(p)&&(r=!0)),a.items.push({type:"list_item",raw:p,task:!!this.options.gfm&&this.rules.other.listIsTask.test(c),loose:!1,text:c,tokens:[]}),a.raw+=p}let o=a.items.at(-1);if(o)o.raw=o.raw.trimEnd(),o.text=o.text.trimEnd();else return;a.raw=a.raw.trimEnd();for(let l of a.items){if(this.lexer.state.top=!1,l.tokens=this.lexer.blockTokens(l.text,[]),l.task){if(l.text=l.text.replace(this.rules.other.listReplaceTask,""),l.tokens[0]?.type==="text"||l.tokens[0]?.type==="paragraph"){l.tokens[0].raw=l.tokens[0].raw.replace(this.rules.other.listReplaceTask,""),l.tokens[0].text=l.tokens[0].text.replace(this.rules.other.listReplaceTask,"");for(let c=this.lexer.inlineQueue.length-1;c>=0;c--)if(this.rules.other.listIsTask.test(this.lexer.inlineQueue[c].src)){this.lexer.inlineQueue[c].src=this.lexer.inlineQueue[c].src.replace(this.rules.other.listReplaceTask,"");break}}let p=this.rules.other.listTaskCheckbox.exec(l.raw);if(p){let c={type:"checkbox",raw:p[0]+" ",checked:p[0]!=="[ ]"};l.checked=c.checked,a.loose?l.tokens[0]&&["paragraph","text"].includes(l.tokens[0].type)&&"tokens"in l.tokens[0]&&l.tokens[0].tokens?(l.tokens[0].raw=c.raw+l.tokens[0].raw,l.tokens[0].text=c.raw+l.tokens[0].text,l.tokens[0].tokens.unshift(c)):l.tokens.unshift({type:"paragraph",raw:c.raw,text:c.raw,tokens:[c]}):l.tokens.unshift(c)}}if(!a.loose){let p=l.tokens.filter(d=>d.type==="space"),c=p.length>0&&p.some(d=>this.rules.other.anyLine.test(d.raw));a.loose=c}}if(a.loose)for(let l of a.items){l.loose=!0;for(let p of l.tokens)p.type==="text"&&(p.type="paragraph")}return a}}html(i){let e=this.rules.block.html.exec(i);if(e)return{type:"html",block:!0,raw:e[0],pre:e[1]==="pre"||e[1]==="script"||e[1]==="style",text:e[0]}}def(i){let e=this.rules.block.def.exec(i);if(e){let t=e[1].toLowerCase().replace(this.rules.other.multipleSpaceGlobal," "),s=e[2]?e[2].replace(this.rules.other.hrefBrackets,"$1").replace(this.rules.inline.anyPunctuation,"$1"):"",a=e[3]?e[3].substring(1,e[3].length-1).replace(this.rules.inline.anyPunctuation,"$1"):e[3];return{type:"def",tag:t,raw:e[0],href:s,title:a}}}table(i){let e=this.rules.block.table.exec(i);if(!e||!this.rules.other.tableDelimiter.test(e[2]))return;let t=De(e[1]),s=e[2].replace(this.rules.other.tableAlignChars,"").split("|"),a=e[3]?.trim()?e[3].replace(this.rules.other.tableRowBlankLine,"").split(`
`):[],n={type:"table",raw:e[0],header:[],align:[],rows:[]};if(t.length===s.length){for(let r of s)this.rules.other.tableAlignRight.test(r)?n.align.push("right"):this.rules.other.tableAlignCenter.test(r)?n.align.push("center"):this.rules.other.tableAlignLeft.test(r)?n.align.push("left"):n.align.push(null);for(let r=0;r<t.length;r++)n.header.push({text:t[r],tokens:this.lexer.inline(t[r]),header:!0,align:n.align[r]});for(let r of a)n.rows.push(De(r,n.header.length).map((o,l)=>({text:o,tokens:this.lexer.inline(o),header:!1,align:n.align[l]})));return n}}lheading(i){let e=this.rules.block.lheading.exec(i);if(e)return{type:"heading",raw:e[0],depth:e[2].charAt(0)==="="?1:2,text:e[1],tokens:this.lexer.inline(e[1])}}paragraph(i){let e=this.rules.block.paragraph.exec(i);if(e){let t=e[1].charAt(e[1].length-1)===`
`?e[1].slice(0,-1):e[1];return{type:"paragraph",raw:e[0],text:t,tokens:this.lexer.inline(t)}}}text(i){let e=this.rules.block.text.exec(i);if(e)return{type:"text",raw:e[0],text:e[0],tokens:this.lexer.inline(e[0])}}escape(i){let e=this.rules.inline.escape.exec(i);if(e)return{type:"escape",raw:e[0],text:e[1]}}tag(i){let e=this.rules.inline.tag.exec(i);if(e)return!this.lexer.state.inLink&&this.rules.other.startATag.test(e[0])?this.lexer.state.inLink=!0:this.lexer.state.inLink&&this.rules.other.endATag.test(e[0])&&(this.lexer.state.inLink=!1),!this.lexer.state.inRawBlock&&this.rules.other.startPreScriptTag.test(e[0])?this.lexer.state.inRawBlock=!0:this.lexer.state.inRawBlock&&this.rules.other.endPreScriptTag.test(e[0])&&(this.lexer.state.inRawBlock=!1),{type:"html",raw:e[0],inLink:this.lexer.state.inLink,inRawBlock:this.lexer.state.inRawBlock,block:!1,text:e[0]}}link(i){let e=this.rules.inline.link.exec(i);if(e){let t=e[2].trim();if(!this.options.pedantic&&this.rules.other.startAngleBracket.test(t)){if(!this.rules.other.endAngleBracket.test(t))return;let n=H(t.slice(0,-1),"\\");if((t.length-n.length)%2===0)return}else{let n=Is(e[2],"()");if(n===-2)return;if(n>-1){let r=(e[0].indexOf("!")===0?5:4)+e[1].length+n;e[2]=e[2].substring(0,n),e[0]=e[0].substring(0,r).trim(),e[3]=""}}let s=e[2],a="";if(this.options.pedantic){let n=this.rules.other.pedanticHrefTitle.exec(s);n&&(s=n[1],a=n[3])}else a=e[3]?e[3].slice(1,-1):"";return s=s.trim(),this.rules.other.startAngleBracket.test(s)&&(this.options.pedantic&&!this.rules.other.endAngleBracket.test(t)?s=s.slice(1):s=s.slice(1,-1)),Fe(e,{href:s&&s.replace(this.rules.inline.anyPunctuation,"$1"),title:a&&a.replace(this.rules.inline.anyPunctuation,"$1")},e[0],this.lexer,this.rules)}}reflink(i,e){let t;if((t=this.rules.inline.reflink.exec(i))||(t=this.rules.inline.nolink.exec(i))){let s=(t[2]||t[1]).replace(this.rules.other.multipleSpaceGlobal," "),a=e[s.toLowerCase()];if(!a){let n=t[0].charAt(0);return{type:"text",raw:n,text:n}}return Fe(t,a,t[0],this.lexer,this.rules)}}emStrong(i,e,t=""){let s=this.rules.inline.emStrongLDelim.exec(i);if(!(!s||s[3]&&t.match(this.rules.other.unicodeAlphaNumeric))&&(!(s[1]||s[2])||!t||this.rules.inline.punctuation.exec(t))){let a=[...s[0]].length-1,n,r,o=a,l=0,p=s[0][0]==="*"?this.rules.inline.emStrongRDelimAst:this.rules.inline.emStrongRDelimUnd;for(p.lastIndex=0,e=e.slice(-1*i.length+a);(s=p.exec(e))!=null;){if(n=s[1]||s[2]||s[3]||s[4]||s[5]||s[6],!n)continue;if(r=[...n].length,s[3]||s[4]){o+=r;continue}else if((s[5]||s[6])&&a%3&&!((a+r)%3)){l+=r;continue}if(o-=r,o>0)continue;r=Math.min(r,r+o+l);let c=[...s[0]][0].length,d=i.slice(0,a+s.index+c+r);if(Math.min(a,r)%2){let u=d.slice(1,-1);return{type:"em",raw:d,text:u,tokens:this.lexer.inlineTokens(u)}}let h=d.slice(2,-2);return{type:"strong",raw:d,text:h,tokens:this.lexer.inlineTokens(h)}}}}codespan(i){let e=this.rules.inline.code.exec(i);if(e){let t=e[2].replace(this.rules.other.newLineCharGlobal," "),s=this.rules.other.nonSpaceChar.test(t),a=this.rules.other.startingSpaceChar.test(t)&&this.rules.other.endingSpaceChar.test(t);return s&&a&&(t=t.substring(1,t.length-1)),{type:"codespan",raw:e[0],text:t}}}br(i){let e=this.rules.inline.br.exec(i);if(e)return{type:"br",raw:e[0]}}del(i){let e=this.rules.inline.del.exec(i);if(e)return{type:"del",raw:e[0],text:e[2],tokens:this.lexer.inlineTokens(e[2])}}autolink(i){let e=this.rules.inline.autolink.exec(i);if(e){let t,s;return e[2]==="@"?(t=e[1],s="mailto:"+t):(t=e[1],s=t),{type:"link",raw:e[0],text:t,href:s,tokens:[{type:"text",raw:t,text:t}]}}}url(i){let e;if(e=this.rules.inline.url.exec(i)){let t,s;if(e[2]==="@")t=e[0],s="mailto:"+t;else{let a;do a=e[0],e[0]=this.rules.inline._backpedal.exec(e[0])?.[0]??"";while(a!==e[0]);t=e[0],e[1]==="www."?s="http://"+e[0]:s=e[0]}return{type:"link",raw:e[0],text:t,href:s,tokens:[{type:"text",raw:t,text:t}]}}}inlineText(i){let e=this.rules.inline.text.exec(i);if(e){let t=this.lexer.state.inRawBlock;return{type:"text",raw:e[0],text:e[0],escaped:t}}}},_=class ie{tokens;options;state;inlineQueue;tokenizer;constructor(e){this.tokens=[],this.tokens.links=Object.create(null),this.options=e||z,this.options.tokenizer=this.options.tokenizer||new Y,this.tokenizer=this.options.tokenizer,this.tokenizer.options=this.options,this.tokenizer.lexer=this,this.inlineQueue=[],this.state={inLink:!1,inRawBlock:!1,top:!0};let t={other:E,block:G.normal,inline:V.normal};this.options.pedantic?(t.block=G.pedantic,t.inline=V.pedantic):this.options.gfm&&(t.block=G.gfm,this.options.breaks?t.inline=V.breaks:t.inline=V.gfm),this.tokenizer.rules=t}static get rules(){return{block:G,inline:V}}static lex(e,t){return new ie(t).lex(e)}static lexInline(e,t){return new ie(t).inlineTokens(e)}lex(e){e=e.replace(E.carriageReturn,`
`),this.blockTokens(e,this.tokens);for(let t=0;t<this.inlineQueue.length;t++){let s=this.inlineQueue[t];this.inlineTokens(s.src,s.tokens)}return this.inlineQueue=[],this.tokens}blockTokens(e,t=[],s=!1){for(this.options.pedantic&&(e=e.replace(E.tabCharGlobal,"    ").replace(E.spaceLine,""));e;){let a;if(this.options.extensions?.block?.some(r=>(a=r.call({lexer:this},e,t))?(e=e.substring(a.raw.length),t.push(a),!0):!1))continue;if(a=this.tokenizer.space(e)){e=e.substring(a.raw.length);let r=t.at(-1);a.raw.length===1&&r!==void 0?r.raw+=`
`:t.push(a);continue}if(a=this.tokenizer.code(e)){e=e.substring(a.raw.length);let r=t.at(-1);r?.type==="paragraph"||r?.type==="text"?(r.raw+=(r.raw.endsWith(`
`)?"":`
`)+a.raw,r.text+=`
`+a.text,this.inlineQueue.at(-1).src=r.text):t.push(a);continue}if(a=this.tokenizer.fences(e)){e=e.substring(a.raw.length),t.push(a);continue}if(a=this.tokenizer.heading(e)){e=e.substring(a.raw.length),t.push(a);continue}if(a=this.tokenizer.hr(e)){e=e.substring(a.raw.length),t.push(a);continue}if(a=this.tokenizer.blockquote(e)){e=e.substring(a.raw.length),t.push(a);continue}if(a=this.tokenizer.list(e)){e=e.substring(a.raw.length),t.push(a);continue}if(a=this.tokenizer.html(e)){e=e.substring(a.raw.length),t.push(a);continue}if(a=this.tokenizer.def(e)){e=e.substring(a.raw.length);let r=t.at(-1);r?.type==="paragraph"||r?.type==="text"?(r.raw+=(r.raw.endsWith(`
`)?"":`
`)+a.raw,r.text+=`
`+a.raw,this.inlineQueue.at(-1).src=r.text):this.tokens.links[a.tag]||(this.tokens.links[a.tag]={href:a.href,title:a.title},t.push(a));continue}if(a=this.tokenizer.table(e)){e=e.substring(a.raw.length),t.push(a);continue}if(a=this.tokenizer.lheading(e)){e=e.substring(a.raw.length),t.push(a);continue}let n=e;if(this.options.extensions?.startBlock){let r=1/0,o=e.slice(1),l;this.options.extensions.startBlock.forEach(p=>{l=p.call({lexer:this},o),typeof l=="number"&&l>=0&&(r=Math.min(r,l))}),r<1/0&&r>=0&&(n=e.substring(0,r+1))}if(this.state.top&&(a=this.tokenizer.paragraph(n))){let r=t.at(-1);s&&r?.type==="paragraph"?(r.raw+=(r.raw.endsWith(`
`)?"":`
`)+a.raw,r.text+=`
`+a.text,this.inlineQueue.pop(),this.inlineQueue.at(-1).src=r.text):t.push(a),s=n.length!==e.length,e=e.substring(a.raw.length);continue}if(a=this.tokenizer.text(e)){e=e.substring(a.raw.length);let r=t.at(-1);r?.type==="text"?(r.raw+=(r.raw.endsWith(`
`)?"":`
`)+a.raw,r.text+=`
`+a.text,this.inlineQueue.pop(),this.inlineQueue.at(-1).src=r.text):t.push(a);continue}if(e){let r="Infinite loop on byte: "+e.charCodeAt(0);if(this.options.silent){console.error(r);break}else throw new Error(r)}}return this.state.top=!0,t}inline(e,t=[]){return this.inlineQueue.push({src:e,tokens:t}),t}inlineTokens(e,t=[]){let s=e,a=null;if(this.tokens.links){let l=Object.keys(this.tokens.links);if(l.length>0)for(;(a=this.tokenizer.rules.inline.reflinkSearch.exec(s))!=null;)l.includes(a[0].slice(a[0].lastIndexOf("[")+1,-1))&&(s=s.slice(0,a.index)+"["+"a".repeat(a[0].length-2)+"]"+s.slice(this.tokenizer.rules.inline.reflinkSearch.lastIndex))}for(;(a=this.tokenizer.rules.inline.anyPunctuation.exec(s))!=null;)s=s.slice(0,a.index)+"++"+s.slice(this.tokenizer.rules.inline.anyPunctuation.lastIndex);let n;for(;(a=this.tokenizer.rules.inline.blockSkip.exec(s))!=null;)n=a[2]?a[2].length:0,s=s.slice(0,a.index+n)+"["+"a".repeat(a[0].length-n-2)+"]"+s.slice(this.tokenizer.rules.inline.blockSkip.lastIndex);s=this.options.hooks?.emStrongMask?.call({lexer:this},s)??s;let r=!1,o="";for(;e;){r||(o=""),r=!1;let l;if(this.options.extensions?.inline?.some(c=>(l=c.call({lexer:this},e,t))?(e=e.substring(l.raw.length),t.push(l),!0):!1))continue;if(l=this.tokenizer.escape(e)){e=e.substring(l.raw.length),t.push(l);continue}if(l=this.tokenizer.tag(e)){e=e.substring(l.raw.length),t.push(l);continue}if(l=this.tokenizer.link(e)){e=e.substring(l.raw.length),t.push(l);continue}if(l=this.tokenizer.reflink(e,this.tokens.links)){e=e.substring(l.raw.length);let c=t.at(-1);l.type==="text"&&c?.type==="text"?(c.raw+=l.raw,c.text+=l.text):t.push(l);continue}if(l=this.tokenizer.emStrong(e,s,o)){e=e.substring(l.raw.length),t.push(l);continue}if(l=this.tokenizer.codespan(e)){e=e.substring(l.raw.length),t.push(l);continue}if(l=this.tokenizer.br(e)){e=e.substring(l.raw.length),t.push(l);continue}if(l=this.tokenizer.del(e)){e=e.substring(l.raw.length),t.push(l);continue}if(l=this.tokenizer.autolink(e)){e=e.substring(l.raw.length),t.push(l);continue}if(!this.state.inLink&&(l=this.tokenizer.url(e))){e=e.substring(l.raw.length),t.push(l);continue}let p=e;if(this.options.extensions?.startInline){let c=1/0,d=e.slice(1),h;this.options.extensions.startInline.forEach(u=>{h=u.call({lexer:this},d),typeof h=="number"&&h>=0&&(c=Math.min(c,h))}),c<1/0&&c>=0&&(p=e.substring(0,c+1))}if(l=this.tokenizer.inlineText(p)){e=e.substring(l.raw.length),l.raw.slice(-1)!=="_"&&(o=l.raw.slice(-1)),r=!0;let c=t.at(-1);c?.type==="text"?(c.raw+=l.raw,c.text+=l.text):t.push(l);continue}if(e){let c="Infinite loop on byte: "+e.charCodeAt(0);if(this.options.silent){console.error(c);break}else throw new Error(c)}}return t}},J=class{options;parser;constructor(i){this.options=i||z}space(i){return""}code({text:i,lang:e,escaped:t}){let s=(e||"").match(E.notSpaceStart)?.[0],a=i.replace(E.endingNewline,"")+`
`;return s?'<pre><code class="language-'+F(s)+'">'+(t?a:F(a,!0))+`</code></pre>
`:"<pre><code>"+(t?a:F(a,!0))+`</code></pre>
`}blockquote({tokens:i}){return`<blockquote>
${this.parser.parse(i)}</blockquote>
`}html({text:i}){return i}def(i){return""}heading({tokens:i,depth:e}){return`<h${e}>${this.parser.parseInline(i)}</h${e}>
`}hr(i){return`<hr>
`}list(i){let e=i.ordered,t=i.start,s="";for(let r=0;r<i.items.length;r++){let o=i.items[r];s+=this.listitem(o)}let a=e?"ol":"ul",n=e&&t!==1?' start="'+t+'"':"";return"<"+a+n+`>
`+s+"</"+a+`>
`}listitem(i){return`<li>${this.parser.parse(i.tokens)}</li>
`}checkbox({checked:i}){return"<input "+(i?'checked="" ':"")+'disabled="" type="checkbox"> '}paragraph({tokens:i}){return`<p>${this.parser.parseInline(i)}</p>
`}table(i){let e="",t="";for(let a=0;a<i.header.length;a++)t+=this.tablecell(i.header[a]);e+=this.tablerow({text:t});let s="";for(let a=0;a<i.rows.length;a++){let n=i.rows[a];t="";for(let r=0;r<n.length;r++)t+=this.tablecell(n[r]);s+=this.tablerow({text:t})}return s&&(s=`<tbody>${s}</tbody>`),`<table>
<thead>
`+e+`</thead>
`+s+`</table>
`}tablerow({text:i}){return`<tr>
${i}</tr>
`}tablecell(i){let e=this.parser.parseInline(i.tokens),t=i.header?"th":"td";return(i.align?`<${t} align="${i.align}">`:`<${t}>`)+e+`</${t}>
`}strong({tokens:i}){return`<strong>${this.parser.parseInline(i)}</strong>`}em({tokens:i}){return`<em>${this.parser.parseInline(i)}</em>`}codespan({text:i}){return`<code>${F(i,!0)}</code>`}br(i){return"<br>"}del({tokens:i}){return`<del>${this.parser.parseInline(i)}</del>`}link({href:i,title:e,tokens:t}){let s=this.parser.parseInline(t),a=Te(i);if(a===null)return s;i=a;let n='<a href="'+i+'"';return e&&(n+=' title="'+F(e)+'"'),n+=">"+s+"</a>",n}image({href:i,title:e,text:t,tokens:s}){s&&(t=this.parser.parseInline(s,this.parser.textRenderer));let a=Te(i);if(a===null)return F(t);i=a;let n=`<img src="${i}" alt="${t}"`;return e&&(n+=` title="${F(e)}"`),n+=">",n}text(i){return"tokens"in i&&i.tokens?this.parser.parseInline(i.tokens):"escaped"in i&&i.escaped?i.text:F(i.text)}},ge=class{strong({text:i}){return i}em({text:i}){return i}codespan({text:i}){return i}del({text:i}){return i}html({text:i}){return i}text({text:i}){return i}link({text:i}){return""+i}image({text:i}){return""+i}br(){return""}checkbox({raw:i}){return i}},M=class ne{options;renderer;textRenderer;constructor(e){this.options=e||z,this.options.renderer=this.options.renderer||new J,this.renderer=this.options.renderer,this.renderer.options=this.options,this.renderer.parser=this,this.textRenderer=new ge}static parse(e,t){return new ne(t).parse(e)}static parseInline(e,t){return new ne(t).parseInline(e)}parse(e){let t="";for(let s=0;s<e.length;s++){let a=e[s];if(this.options.extensions?.renderers?.[a.type]){let r=a,o=this.options.extensions.renderers[r.type].call({parser:this},r);if(o!==!1||!["space","hr","heading","code","table","blockquote","list","html","def","paragraph","text"].includes(r.type)){t+=o||"";continue}}let n=a;switch(n.type){case"space":{t+=this.renderer.space(n);break}case"hr":{t+=this.renderer.hr(n);break}case"heading":{t+=this.renderer.heading(n);break}case"code":{t+=this.renderer.code(n);break}case"table":{t+=this.renderer.table(n);break}case"blockquote":{t+=this.renderer.blockquote(n);break}case"list":{t+=this.renderer.list(n);break}case"checkbox":{t+=this.renderer.checkbox(n);break}case"html":{t+=this.renderer.html(n);break}case"def":{t+=this.renderer.def(n);break}case"paragraph":{t+=this.renderer.paragraph(n);break}case"text":{t+=this.renderer.text(n);break}default:{let r='Token with "'+n.type+'" type was not found.';if(this.options.silent)return console.error(r),"";throw new Error(r)}}}return t}parseInline(e,t=this.renderer){let s="";for(let a=0;a<e.length;a++){let n=e[a];if(this.options.extensions?.renderers?.[n.type]){let o=this.options.extensions.renderers[n.type].call({parser:this},n);if(o!==!1||!["escape","html","link","image","strong","em","codespan","br","del","text"].includes(n.type)){s+=o||"";continue}}let r=n;switch(r.type){case"escape":{s+=t.text(r);break}case"html":{s+=t.html(r);break}case"link":{s+=t.link(r);break}case"image":{s+=t.image(r);break}case"checkbox":{s+=t.checkbox(r);break}case"strong":{s+=t.strong(r);break}case"em":{s+=t.em(r);break}case"codespan":{s+=t.codespan(r);break}case"br":{s+=t.br(r);break}case"del":{s+=t.del(r);break}case"text":{s+=t.text(r);break}default:{let o='Token with "'+r.type+'" type was not found.';if(this.options.silent)return console.error(o),"";throw new Error(o)}}}return s}},U=class{options;block;constructor(i){this.options=i||z}static passThroughHooks=new Set(["preprocess","postprocess","processAllTokens","emStrongMask"]);static passThroughHooksRespectAsync=new Set(["preprocess","postprocess","processAllTokens"]);preprocess(i){return i}postprocess(i){return i}processAllTokens(i){return i}emStrongMask(i){return i}provideLexer(){return this.block?_.lex:_.lexInline}provideParser(){return this.block?M.parse:M.parseInline}},Ts=class{defaults=le();options=this.setOptions;parse=this.parseMarkdown(!0);parseInline=this.parseMarkdown(!1);Parser=M;Renderer=J;TextRenderer=ge;Lexer=_;Tokenizer=Y;Hooks=U;constructor(...i){this.use(...i)}walkTokens(i,e){let t=[];for(let s of i)switch(t=t.concat(e.call(this,s)),s.type){case"table":{let a=s;for(let n of a.header)t=t.concat(this.walkTokens(n.tokens,e));for(let n of a.rows)for(let r of n)t=t.concat(this.walkTokens(r.tokens,e));break}case"list":{let a=s;t=t.concat(this.walkTokens(a.items,e));break}default:{let a=s;this.defaults.extensions?.childTokens?.[a.type]?this.defaults.extensions.childTokens[a.type].forEach(n=>{let r=a[n].flat(1/0);t=t.concat(this.walkTokens(r,e))}):a.tokens&&(t=t.concat(this.walkTokens(a.tokens,e)))}}return t}use(...i){let e=this.defaults.extensions||{renderers:{},childTokens:{}};return i.forEach(t=>{let s={...t};if(s.async=this.defaults.async||s.async||!1,t.extensions&&(t.extensions.forEach(a=>{if(!a.name)throw new Error("extension name required");if("renderer"in a){let n=e.renderers[a.name];n?e.renderers[a.name]=function(...r){let o=a.renderer.apply(this,r);return o===!1&&(o=n.apply(this,r)),o}:e.renderers[a.name]=a.renderer}if("tokenizer"in a){if(!a.level||a.level!=="block"&&a.level!=="inline")throw new Error("extension level must be 'block' or 'inline'");let n=e[a.level];n?n.unshift(a.tokenizer):e[a.level]=[a.tokenizer],a.start&&(a.level==="block"?e.startBlock?e.startBlock.push(a.start):e.startBlock=[a.start]:a.level==="inline"&&(e.startInline?e.startInline.push(a.start):e.startInline=[a.start]))}"childTokens"in a&&a.childTokens&&(e.childTokens[a.name]=a.childTokens)}),s.extensions=e),t.renderer){let a=this.defaults.renderer||new J(this.defaults);for(let n in t.renderer){if(!(n in a))throw new Error(`renderer '${n}' does not exist`);if(["options","parser"].includes(n))continue;let r=n,o=t.renderer[r],l=a[r];a[r]=(...p)=>{let c=o.apply(a,p);return c===!1&&(c=l.apply(a,p)),c||""}}s.renderer=a}if(t.tokenizer){let a=this.defaults.tokenizer||new Y(this.defaults);for(let n in t.tokenizer){if(!(n in a))throw new Error(`tokenizer '${n}' does not exist`);if(["options","rules","lexer"].includes(n))continue;let r=n,o=t.tokenizer[r],l=a[r];a[r]=(...p)=>{let c=o.apply(a,p);return c===!1&&(c=l.apply(a,p)),c}}s.tokenizer=a}if(t.hooks){let a=this.defaults.hooks||new U;for(let n in t.hooks){if(!(n in a))throw new Error(`hook '${n}' does not exist`);if(["options","block"].includes(n))continue;let r=n,o=t.hooks[r],l=a[r];U.passThroughHooks.has(n)?a[r]=p=>{if(this.defaults.async&&U.passThroughHooksRespectAsync.has(n))return(async()=>{let d=await o.call(a,p);return l.call(a,d)})();let c=o.call(a,p);return l.call(a,c)}:a[r]=(...p)=>{if(this.defaults.async)return(async()=>{let d=await o.apply(a,p);return d===!1&&(d=await l.apply(a,p)),d})();let c=o.apply(a,p);return c===!1&&(c=l.apply(a,p)),c}}s.hooks=a}if(t.walkTokens){let a=this.defaults.walkTokens,n=t.walkTokens;s.walkTokens=function(r){let o=[];return o.push(n.call(this,r)),a&&(o=o.concat(a.call(this,r))),o}}this.defaults={...this.defaults,...s}}),this}setOptions(i){return this.defaults={...this.defaults,...i},this}lexer(i,e){return _.lex(i,e??this.defaults)}parser(i,e){return M.parse(i,e??this.defaults)}parseMarkdown(i){return(e,t)=>{let s={...t},a={...this.defaults,...s},n=this.onError(!!a.silent,!!a.async);if(this.defaults.async===!0&&s.async===!1)return n(new Error("marked(): The async option was set to true by an extension. Remove async: false from the parse options object to return a Promise."));if(typeof e>"u"||e===null)return n(new Error("marked(): input parameter is undefined or null"));if(typeof e!="string")return n(new Error("marked(): input parameter is of type "+Object.prototype.toString.call(e)+", string expected"));if(a.hooks&&(a.hooks.options=a,a.hooks.block=i),a.async)return(async()=>{let r=a.hooks?await a.hooks.preprocess(e):e,o=await(a.hooks?await a.hooks.provideLexer():i?_.lex:_.lexInline)(r,a),l=a.hooks?await a.hooks.processAllTokens(o):o;a.walkTokens&&await Promise.all(this.walkTokens(l,a.walkTokens));let p=await(a.hooks?await a.hooks.provideParser():i?M.parse:M.parseInline)(l,a);return a.hooks?await a.hooks.postprocess(p):p})().catch(n);try{a.hooks&&(e=a.hooks.preprocess(e));let r=(a.hooks?a.hooks.provideLexer():i?_.lex:_.lexInline)(e,a);a.hooks&&(r=a.hooks.processAllTokens(r)),a.walkTokens&&this.walkTokens(r,a.walkTokens);let o=(a.hooks?a.hooks.provideParser():i?M.parse:M.parseInline)(r,a);return a.hooks&&(o=a.hooks.postprocess(o)),o}catch(r){return n(r)}}}onError(i,e){return t=>{if(t.message+=`
Please report this to https://github.com/markedjs/marked.`,i){let s="<p>An error occurred:</p><pre>"+F(t.message+"",!0)+"</pre>";return e?Promise.resolve(s):s}if(e)return Promise.reject(t);throw t}}},q=new Ts;function x(i,e){return q.parse(i,e)}x.options=x.setOptions=function(i){return q.setOptions(i),x.defaults=q.defaults,Ne(x.defaults),x};x.getDefaults=le;x.defaults=z;x.use=function(...i){return q.use(...i),x.defaults=q.defaults,Ne(x.defaults),x};x.walkTokens=function(i,e){return q.walkTokens(i,e)};x.parseInline=q.parseInline;x.Parser=M;x.parser=M.parse;x.Renderer=J;x.TextRenderer=ge;x.Lexer=_;x.lexer=_.lex;x.Tokenizer=Y;x.Hooks=U;x.parse=x;x.options;x.setOptions;x.use;x.walkTokens;x.parseInline;M.parse;_.lex;x.use({renderer:{heading({tokens:i,depth:e}){const t=this.parser.parseInline(i),s=t.replace(/^\d+\.\s*/,"").toLowerCase().replace(/[^\w]+/g,"-").replace(/(^-|-$)/g,"");return`<h${e} id="${s}">${t}</h${e}>
`}}});class Ds extends HTMLElement{api=null;dbc=$e();currentFile=null;isDirty=!1;selectedMessageId=null;selectedMessageExtended=!1;isAddingMessage=!1;activeTab="messages";specificationContent=null;isEditMode=!1;dbcBeforeEdit=null;frames=[];handleMdf4Changed=e=>this.onMdf4Changed(e);constructor(){super(),this.attachShadow({mode:"open"})}connectedCallback(){this.render(),f.on("mdf4:changed",this.handleMdf4Changed),this.emitStateChange()}disconnectedCallback(){f.off("mdf4:changed",this.handleMdf4Changed)}onMdf4Changed(e){e.action==="loaded"||e.action==="capture-stopped"?this.frames=k.get().mdf4Frames:e.action==="cleared"&&(this.frames=[])}setApi(e){this.api=e,this.loadInitialState()}setFrames(e){this.frames=e}getDbc(){return this.dbc}hasUnsavedChanges(){return this.isDirty}emitStateChange(){ot({isDirty:this.isDirty,isEditing:this.isEditMode,currentFile:this.currentFile,messageCount:this.dbc.messages.length}),k.set({dbcFile:this.currentFile})}async loadFile(e){if(this.api)try{this.dbc=await this.api.loadDbc(e),this.currentFile=e,this.isDirty=!1,this.selectedMessageId=null,this.isAddingMessage=!1,this.render(),this.emitStateChange(),O({action:"loaded",dbcInfo:null,filename:e.split("/").pop()||null}),this.showToast("File loaded successfully","success")}catch(t){this.showToast(`Failed to load file: ${t}`,"error")}}async loadInitialState(){if(this.api)try{const e=await this.api.getDbc();e&&(this.dbc=e,this.currentFile=await this.api.getCurrentFile(),this.isDirty=await this.api.isDirty(),this.render(),this.emitStateChange())}catch{}}render(){this.shadowRoot&&(this.shadowRoot.innerHTML=`
      <style>${L}
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
            <button class="cv-tab ${this.activeTab==="messages"?"active":""}" data-tab="messages">
              Messages <span class="cv-tab-badge">${this.dbc.messages.length}</span>
            </button>
            <button class="cv-tab ${this.activeTab==="nodes"?"active":""}" data-tab="nodes">
              Nodes <span class="cv-tab-badge">${this.dbc.nodes.length}</span>
            </button>
            <button class="cv-tab ${this.activeTab==="values"?"active":""}" data-tab="values">
              Values <span class="cv-tab-badge">${this.dbc.value_descriptions.length}</span>
            </button>
            <button class="cv-tab ${this.activeTab==="attributes"?"active":""}" data-tab="attributes">
              Attributes <span class="cv-tab-badge">${this.dbc.attribute_definitions.length}</span>
            </button>
            <button class="cv-tab ${this.activeTab==="timing"?"active":""}" data-tab="timing">
              Bit Timing
            </button>
            <button class="cv-tab ${this.activeTab==="version"?"active":""}" data-tab="version">
              Version
            </button>
            <button class="cv-tab ${this.activeTab==="preview"?"active":""}" data-tab="preview">
              Preview
            </button>
            <button class="cv-tab ${this.activeTab==="reference"?"active":""}" data-tab="reference">
              Reference
            </button>
          </div>
        </div>

        <div class="cv-editor-main">
          ${this.activeTab==="messages"?this.renderMessagesTab():""}
          ${this.activeTab==="nodes"?this.renderNodesTab():""}
          ${this.activeTab==="values"?this.renderValuesTab():""}
          ${this.activeTab==="attributes"?this.renderAttributesTab():""}
          ${this.activeTab==="timing"?this.renderTimingTab():""}
          ${this.activeTab==="version"?this.renderVersionTab():""}
          ${this.activeTab==="preview"?this.renderPreviewTab():""}
          ${this.activeTab==="reference"?this.renderReferenceTab():""}
        </div>
      </div>
    `,this.setupEventListeners(),this.updateChildComponents())}renderMessagesTab(){const e=this.isAddingMessage?null:this.dbc.messages.find(t=>t.id===this.selectedMessageId&&t.is_extended===this.selectedMessageExtended);return`
      <div class="cv-grid responsive">
        <cv-messages-list class="cv-card" id="messagesList">
          <div class="cv-card-header">
            <span class="cv-card-title">Messages <span class="cv-tab-badge">${this.dbc.messages.length}</span></span>
            ${this.isEditMode?'<button class="cv-btn accent small" id="add-message-btn">+ Add</button>':""}
          </div>
        </cv-messages-list>

        <div class="cv-card cv-card-no-header" id="messageDetail">
          ${this.isAddingMessage||e?`
            <cv-message-editor data-edit-mode="${this.isEditMode}"></cv-message-editor>
          `:`
            <div class="cv-empty-state">
              <div class="cv-empty-state-title">No Message Selected</div>
              <p>Select a message from the list to ${this.isEditMode?'edit it, or click "Add" to create a new one':"view its details"}.</p>
            </div>
          `}
        </div>
      </div>
    `}renderNodesTab(){return`
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
    `}renderValuesTab(){return`
      <div class="cv-grid" style="justify-content: center;">
        <div class="cv-card" style="max-width: 800px;">
          <div class="cv-card-header">
            <span class="cv-card-title">Value Descriptions</span>
          </div>
          <div class="cv-card-body padded">
            <p class="cv-help-text">
              Define value-to-text mappings for signals. These appear as VAL_ statements and are used to decode enumerated signal values.
            </p>
            <cv-value-descriptions-editor></cv-value-descriptions-editor>
          </div>
        </div>
      </div>
    `}renderAttributesTab(){return`
      <div class="cv-grid" style="justify-content: center;">
        <div class="cv-card" style="max-width: 800px;">
          <div class="cv-card-header">
            <span class="cv-card-title">Attributes</span>
          </div>
          <div class="cv-card-body padded">
            <p class="cv-help-text">
              Define and assign attributes to DBC objects. Attributes provide metadata like cycle times, send types, and custom properties.
            </p>
            <cv-attributes-editor></cv-attributes-editor>
          </div>
        </div>
      </div>
    `}renderTimingTab(){const e=this.dbc.bit_timing;return`
      <div class="cv-grid" style="justify-content: center;">
        <div class="cv-card" style="max-width: 600px;">
          <div class="cv-card-header">
            <span class="cv-card-title">Bit Timing Configuration</span>
          </div>
          <div class="cv-card-body padded">
            <p class="cv-help-text">
              Configure the CAN bus bit timing parameters. These appear in the BS_ section of the DBC file.
              Note: Most tools ignore this section and configure bit timing at the interface level.
            </p>
            <div class="cv-form-group">
              <label class="cv-label">Baud Rate (bps)</label>
              <select class="cv-select ${e&&e.baudrate>0?"has-value":""}" style="max-width: 200px" id="bit-timing-baudrate">
                <option value="0" ${!e||e.baudrate===0?"selected":""}>Not specified</option>
                <option value="125000" ${e?.baudrate===125e3?"selected":""}>125 kbps</option>
                <option value="250000" ${e?.baudrate===25e4?"selected":""}>250 kbps</option>
                <option value="500000" ${e?.baudrate===5e5?"selected":""}>500 kbps</option>
                <option value="1000000" ${e?.baudrate===1e6?"selected":""}>1 Mbps</option>
              </select>
            </div>
            <div class="cv-form-group">
              <label class="cv-label">BTR1 (Bit Timing Register 1)</label>
              <input type="number" class="cv-input" style="max-width: 100px" id="bit-timing-btr1"
                     value="${e?.btr1??0}" min="0" max="255"
                     ${!e||e.baudrate===0?"disabled":""}>
            </div>
            <div class="cv-form-group">
              <label class="cv-label">BTR2 (Bit Timing Register 2)</label>
              <input type="number" class="cv-input" style="max-width: 100px" id="bit-timing-btr2"
                     value="${e?.btr2??0}" min="0" max="255"
                     ${!e||e.baudrate===0?"disabled":""}>
            </div>
          </div>
        </div>
      </div>
    `}renderVersionTab(){return`
      <div class="cv-grid" style="justify-content: center;">
        <div class="cv-card" style="max-width: 600px;">
          <div class="cv-card-header">
            <span class="cv-card-title">DBC Version</span>
          </div>
          <div class="cv-card-body padded">
            <p class="cv-help-text">
              Set the version string for this DBC file (e.g., major.minor.patch). This appears in the VERSION statement at the top of the file.
            </p>
            <div class="cv-form-group">
              <label class="cv-label">Version</label>
              <input type="text" class="cv-input" style="max-width: 200px" id="dbc-version"
                     value="${this.dbc.version||""}"
                     placeholder="e.g., 1.0.0">
            </div>
          </div>
        </div>
      </div>
    `}renderPreviewTab(){const e=se(this.dbc);return`
      <div class="cv-grid">
        <div class="cv-card">
          <div class="cv-card-header">
            <span class="cv-card-title">DBC File Preview</span>
          </div>
          <div class="cv-preview-content">
            <pre class="cv-preview-text">${g(e)}</pre>
          </div>
        </div>
      </div>
    `}renderReferenceTab(){return this.specificationContent===null?(this.loadSpecification(),`
        <div class="cv-grid" style="justify-content: center;">
          <div class="cv-card" style="max-width: 900px;">
            <div class="cv-card-header">
              <span class="cv-card-title">DBC File Format Reference</span>
            </div>
            <div class="cv-card-body padded">
              <p>Loading specification...</p>
            </div>
          </div>
        </div>
      `):`
      <div class="cv-grid" style="justify-content: center;">
        <div class="cv-card" style="max-width: 900px;">
          <div class="cv-card-header">
            <span class="cv-card-title">DBC File Format Reference</span>
          </div>
          <div class="cv-markdown-content">
            ${x.parse(this.specificationContent)}
          </div>
        </div>
      </div>
    `}async loadSpecification(){if(this.api?.getDbcSpecification)try{this.specificationContent=await this.api.getDbcSpecification(),this.render()}catch(e){this.specificationContent=`Failed to load specification: ${e}`,this.render()}else this.specificationContent="Specification not available (API method not implemented)",this.render()}setupEventListeners(){if(!this.shadowRoot)return;this.shadowRoot.querySelectorAll(".cv-tab").forEach(h=>{h.addEventListener("click",()=>{this.activeTab=h.dataset.tab,this.render()})}),this.shadowRoot.getElementById("add-message-btn")?.addEventListener("click",()=>{this.isAddingMessage=!0,this.selectedMessageId=null,this.render()}),this.shadowRoot.querySelector("cv-messages-list")?.addEventListener("message-select",(h=>{this.selectedMessageId=h.detail.id,this.selectedMessageExtended=h.detail.isExtended,this.isAddingMessage=!1,this.render()}));const t=this.shadowRoot.querySelector("cv-message-editor");t?.addEventListener("message-edit-done",(()=>{this.handleSaveMessage()})),t?.addEventListener("message-delete-request",(()=>{this.handleDeleteMessage()})),t?.addEventListener("message-edit-cancel",(()=>{this.isAddingMessage=!1,this.selectedMessageId=null,this.render()})),t?.addEventListener("message-change",(h=>{const u=h,v=this.dbc.messages.findIndex(m=>m.id===this.selectedMessageId&&m.is_extended===this.selectedMessageExtended);v>=0&&(this.dbc.messages[v]=u.detail,this.isDirty=!0,this.syncToBackend(),this.emitStateChange())})),this.shadowRoot.querySelector("cv-nodes-editor")?.addEventListener("nodes-change",(h=>{const u=h;this.dbc.nodes=u.detail.nodes,this.isDirty=!0,this.syncToBackend(),this.emitStateChange(),this.render()}));const a=this.shadowRoot.getElementById("dbc-version");a?.addEventListener("input",async()=>{this.dbc.version=a.value||null,this.isDirty=!0,await this.syncToBackend(),this.emitStateChange()});const n=this.shadowRoot.getElementById("bit-timing-baudrate"),r=this.shadowRoot.getElementById("bit-timing-btr1"),o=this.shadowRoot.getElementById("bit-timing-btr2"),l=async()=>{const h=parseInt(n?.value||"0",10);h===0?this.dbc.bit_timing=null:this.dbc.bit_timing={baudrate:h,btr1:parseInt(r?.value||"0",10),btr2:parseInt(o?.value||"0",10)},this.isDirty=!0,await this.syncToBackend(),this.emitStateChange()};n?.addEventListener("change",async()=>{const h=parseInt(n.value,10);r&&(r.disabled=h===0),o&&(o.disabled=h===0),n.classList.toggle("has-value",h>0),await l()}),r?.addEventListener("input",l),o?.addEventListener("input",l),this.shadowRoot.querySelector("cv-value-descriptions-editor")?.addEventListener("value-descriptions-change",(h=>{const u=h;this.dbc.value_descriptions=u.detail.valueDescriptions,this.isDirty=!0,this.syncToBackend(),this.emitStateChange()})),this.shadowRoot.querySelector("cv-attributes-editor")?.addEventListener("attributes-change",(h=>{const u=h;this.dbc.attribute_definitions=u.detail.definitions,this.dbc.attribute_defaults=u.detail.defaults,this.dbc.attribute_values=u.detail.values,this.isDirty=!0,this.syncToBackend(),this.emitStateChange()}));const d=this.shadowRoot.querySelector(".cv-markdown-content");d?.addEventListener("click",h=>{const u=h.target;if(u.tagName==="A"){const v=u.getAttribute("href");if(v?.startsWith("#")){h.preventDefault();const m=v.slice(1);d.querySelector(`#${CSS.escape(m)}`)?.scrollIntoView({behavior:"smooth",block:"start"})}}})}updateChildComponents(){if(!this.shadowRoot)return;const e=this.shadowRoot.querySelector("cv-messages-list");e&&(e.setMessages(this.dbc.messages),e.setSelected(this.selectedMessageId,this.selectedMessageExtended));const t=this.shadowRoot.querySelector("cv-message-editor");if(t){const r=this.isAddingMessage?Q():this.dbc.messages.find(o=>o.id===this.selectedMessageId&&o.is_extended===this.selectedMessageExtended)||null;t.setMessage(r,this.isAddingMessage),t.setAvailableNodes(this.dbc.nodes.map(o=>o.name)),t.setFrames(this.frames)}const s=this.shadowRoot.querySelector("cv-nodes-editor");s&&s.setNodes(this.dbc.nodes);const a=this.shadowRoot.querySelector("cv-value-descriptions-editor");a&&(a.setValueDescriptions(this.dbc.value_descriptions),a.setMessages(this.dbc.messages));const n=this.shadowRoot.querySelector("cv-attributes-editor");n&&(n.setData({definitions:this.dbc.attribute_definitions,defaults:this.dbc.attribute_defaults,values:this.dbc.attribute_values}),n.setMessages(this.dbc.messages),n.setNodes(this.dbc.nodes))}setEditMode(e){e&&!this.isEditMode&&(this.dbcBeforeEdit=I(this.dbc)),this.isEditMode=e,e||(this.isAddingMessage=!1,this.dbcBeforeEdit=null),this.render(),this.emitStateChange()}cancelEdit(){this.dbcBeforeEdit&&(this.dbc=this.dbcBeforeEdit,this.dbcBeforeEdit=null,this.isDirty=!1,this.syncToBackend()),this.isEditMode=!1,this.isAddingMessage=!1,this.selectedMessageId=null,this.render(),this.emitStateChange()}getEditMode(){return this.isEditMode}getIsDirty(){return this.isDirty}getCurrentFile(){return this.currentFile}getMessageCount(){return this.dbc.messages.length}async handleNew(){if(!(this.isDirty&&!confirm("You have unsaved changes. Create a new file anyway?")))if(this.api)try{this.dbc=await this.api.newDbc(),this.currentFile=null,this.isDirty=!1,this.selectedMessageId=null,this.isAddingMessage=!1,this.render(),this.emitStateChange(),O({action:"new",dbcInfo:null,filename:null}),this.showToast("New DBC created","success")}catch(e){this.showToast(`Failed to create new DBC: ${e}`,"error")}else this.dbc=$e(),this.currentFile=null,this.isDirty=!1,this.selectedMessageId=null,this.isAddingMessage=!1,this.render(),this.emitStateChange(),O({action:"new",dbcInfo:null,filename:null})}async handleOpen(){if(this.api&&!(this.isDirty&&!confirm("You have unsaved changes. Open a different file anyway?")))try{const e=await this.api.openFileDialog();e&&await this.loadFile(e)}catch(e){this.showToast(`Failed to open file: ${e}`,"error")}}async handleSave(){this.api&&(this.currentFile?await this.saveToPath(this.currentFile):await this.handleSaveAs())}async handleSaveAs(){if(this.api)try{const e=await this.api.saveFileDialog(this.currentFile||void 0);e&&await this.saveToPath(e)}catch(e){this.showToast(`Failed to save file: ${e}`,"error")}}async saveToPath(e){if(this.api)try{const t=se(this.dbc);await this.api.saveDbcContent(e,t),await this.api.loadDbc(e),this.currentFile=e,this.isDirty=!1,this.isEditMode=!1,this.isAddingMessage=!1,this.dbcBeforeEdit=null,this.render(),this.emitStateChange(),O({action:"updated",dbcInfo:null,filename:e.split("/").pop()||null}),this.showToast("File saved successfully","success")}catch(t){this.showToast(`Failed to save file: ${t}`,"error")}}async handleSaveMessage(){const e=this.shadowRoot?.querySelector("de-message-editor");if(!e)return;const t=e.getMessage();if(!t.name){this.showToast("Message name is required","error");return}if(this.isAddingMessage){if(this.dbc.messages.some(s=>s.id===t.id&&s.is_extended===t.is_extended)){this.showToast(`Message with ID ${t.id} already exists`,"error");return}this.dbc.messages.push(t),this.selectedMessageId=t.id,this.selectedMessageExtended=t.is_extended}else{const s=this.dbc.messages.findIndex(a=>a.id===this.selectedMessageId&&a.is_extended===this.selectedMessageExtended);if(s>=0){if((t.id!==this.selectedMessageId||t.is_extended!==this.selectedMessageExtended)&&this.dbc.messages.some(a=>a.id===t.id&&a.is_extended===t.is_extended)){this.showToast(`Message with ID ${t.id} already exists`,"error");return}this.dbc.messages[s]=t,this.selectedMessageId=t.id,this.selectedMessageExtended=t.is_extended}}this.isDirty=!0,this.isAddingMessage=!1,await this.syncToBackend(),this.render(),this.emitStateChange(),this.showToast("Message saved","success")}async handleDeleteMessage(){this.selectedMessageId!==null&&(this.dbc.messages=this.dbc.messages.filter(e=>!(e.id===this.selectedMessageId&&e.is_extended===this.selectedMessageExtended)),this.selectedMessageId=null,this.isDirty=!0,await this.syncToBackend(),this.render(),this.emitStateChange(),this.showToast("Message deleted","success"))}async syncToBackend(){if(this.api)try{await this.api.updateDbc(this.dbc)}catch(e){console.error("Failed to sync to backend:",e)}}showToast(e,t){const s=document.createElement("div");s.className=`cv-toast ${t}`,s.textContent=e,this.shadowRoot?.appendChild(s),setTimeout(()=>{s.remove()},3e3)}}customElements.define("cv-dbc-editor",Ds);const Ae="can-viewer:active-tab";class Fs extends HTMLElement{api=null;config;state;shadow;mdf4Inspector=null;liveViewer=null;dbcEditor=null;extensions=[];aboutExtensions=[];boundBeforeUnload=this.handleBeforeUnload.bind(this);handleTabSwitch=e=>this.switchTab(e.tab);constructor(){super(),this.config={...we};const e=localStorage.getItem(Ae);this.state={activeTab:e||this.config.initialTab,dbcLoaded:!1,dbcFilename:null},this.shadow=this.attachShadow({mode:"open"})}setApi(e){this.api=e,this.setupComponents(),this.loadInitialFiles()}setConfig(e){this.config={...we,...e},this.state.activeTab=this.config.initialTab,this.render()}configure(e){this.config={...this.config,...e},this.isConnected&&this.render()}async registerExtension(e){this.extensions.push(e),e.setup&&this.api&&await e.setup(this.api),this.isConnected&&this.render()}addAboutTab(e,t,s){this.aboutExtensions.push({id:e,label:t,panel:s}),this.isConnected&&this.render()}connectedCallback(){this.render(),this.loadVersion(),window.addEventListener("beforeunload",this.boundBeforeUnload),f.on("tab:switch",this.handleTabSwitch)}async loadVersion(){try{const e=await $t(),t=this.shadow.querySelector("#appVersion");t&&(t.textContent=`v${e}`)}catch{}}disconnectedCallback(){window.removeEventListener("beforeunload",this.boundBeforeUnload),f.off("tab:switch",this.handleTabSwitch)}handleBeforeUnload(e){this.dbcEditor?.getIsDirty()&&(e.preventDefault(),e.returnValue="You have unsaved DBC changes. Are you sure you want to leave?")}render(){this.shadow.innerHTML=`
      <style>${L}</style>
      ${this.generateTemplate()}
    `,this.cacheElements(),this.bindEvents(),this.switchTab(this.state.activeTab)}generateTemplate(){const e=this.extensions.filter(s=>s.tab).map(s=>`<button class="cv-tab" data-tab="${s.tab.id}" title="${s.tab.title||""}">${s.tab.icon||""}${s.tab.label}</button>`).join(""),t=this.extensions.filter(s=>s.panel).map(s=>`<${s.panel} class="cv-panel hidden" id="${s.tab?.id||s.id}Panel"></${s.panel}>`).join("");return`
      <div class="cv-app">
        <header class="cv-app-header">
          <div class="cv-header-row">
            <h1 class="cv-app-title">${this.config.appName}</h1>
            <div class="cv-header-status">
              <cv-dbc-status></cv-dbc-status>
              <cv-mdf4-status></cv-mdf4-status>
            </div>
          </div>
          <nav class="cv-tabs bordered">
            ${this.config.showDbcTab?'<button class="cv-tab" data-tab="dbc" title="View and manage DBC files">DBC</button>':""}
            ${this.config.showMdf4Tab?'<button class="cv-tab" data-tab="mdf4" title="Load MDF4 measurement files">MDF4</button>':""}
            ${this.config.showLiveTab?'<button class="cv-tab" data-tab="live" title="Capture from SocketCAN">Live</button>':""}
            ${e}
            ${this.config.showAboutTab?`<button class="cv-tab" data-tab="about" title="About ${this.config.appName}">About</button>`:""}
          </nav>
          <cv-mdf4-toolbar></cv-mdf4-toolbar>
          <cv-live-toolbar></cv-live-toolbar>
          <cv-dbc-toolbar></cv-dbc-toolbar>
          <div id="aboutTab" class="cv-toolbar cv-tab-pane cv-about-header">
            <span class="cv-about-title">${this.config.appName}</span>
            <span class="cv-about-version" id="appVersion"></span>
            <span class="cv-about-desc">A desktop application for viewing and analyzing CAN bus data from MDF4 files and live SocketCAN interfaces.</span>
          </div>
        </header>
        <cv-mdf4-inspector class="cv-panel hidden" id="mdf4Panel"></cv-mdf4-inspector>
        <cv-live-viewer class="cv-panel hidden" id="livePanel"></cv-live-viewer>
        <cv-dbc-editor class="cv-panel hidden" id="dbcPanel"></cv-dbc-editor>
        ${t}
        ${this.generateAboutPanel()}
      </div>
    `}generateAboutPanel(){const e=this.aboutExtensions.map(s=>`<button class="cv-tab" data-tab="${s.id}">${s.label}</button>`).join(""),t=this.aboutExtensions.map(s=>`<div class="cv-tab-pane" id="about${s.id.charAt(0).toUpperCase()+s.id.slice(1)}"><${s.panel}></${s.panel}></div>`).join("");return`
      <section class="cv-panel hidden" id="aboutPanel">
        <nav class="cv-panel-header cv-tabs">
          <button class="cv-tab active" data-tab="features">Features</button>
          <button class="cv-tab" data-tab="acknowledgments">Acknowledgments</button>
          ${e}
        </nav>
        <div class="cv-tab-pane active" id="aboutFeatures">
          <div class="cv-grid responsive">
            <div class="cv-card"><div class="cv-card-header"><span class="cv-card-title">MDF4 File Support</span></div><p class="cv-card-body">Load and analyze CAN data from ASAM MDF4 measurement files with timestamps and decoded signals.</p></div>
            <div class="cv-card"><div class="cv-card-header"><span class="cv-card-title">Live SocketCAN Capture</span></div><p class="cv-card-body">Lossless capture from Linux SocketCAN with real-time MDF4 recording and live monitors.</p></div>
            <div class="cv-card"><div class="cv-card-header"><span class="cv-card-title">DBC Signal Decoding</span></div><p class="cv-card-body">Decode raw CAN frames into physical values. Supports CAN 2.0 and CAN FD with extended IDs.</p></div>
            <div class="cv-card"><div class="cv-card-header"><span class="cv-card-title">Built-in DBC Editor</span></div><p class="cv-card-body">Create and modify DBC files directly. Edit messages, signals, and their properties.</p></div>
            <div class="cv-card"><div class="cv-card-header"><span class="cv-card-title">Real-time Monitors</span></div><p class="cv-card-body">Message monitor shows latest data per CAN ID. Signal monitor groups decoded values by message.</p></div>
            <div class="cv-card"><div class="cv-card-header"><span class="cv-card-title">High Performance</span></div><p class="cv-card-body">Rust backend handles all processing. Pre-rendered updates minimize frontend overhead.</p></div>
          </div>
        </div>
        <div class="cv-tab-pane" id="aboutAcknowledgments">
          <div class="cv-grid responsive">
            <div class="cv-card"><div class="cv-card-header"><span class="cv-card-title">Standards</span></div><ul class="cv-card-body cv-deps-list"><li><a href="https://www.asam.net/standards/detail/mdf/" target="_blank">ASAM MDF4</a> – Measurement data format</li><li><a href="https://docs.kernel.org/networking/can.html" target="_blank">SocketCAN</a> – Linux CAN subsystem</li><li><a href="https://www.iso.org/standard/63648.html" target="_blank">ISO 11898</a> – CAN protocol spec</li></ul></div>
            <div class="cv-card"><div class="cv-card-header"><span class="cv-card-title">Rust Core</span></div><ul class="cv-card-body cv-deps-list"><li><a href="https://tauri.app" target="_blank">Tauri</a> – Desktop app framework</li><li class="cv-sister-project"><a href="https://crates.io/crates/mdf4-rs" target="_blank">mdf4-rs</a> – MDF4 parser/writer</li><li class="cv-sister-project"><a href="https://crates.io/crates/dbc-rs" target="_blank">dbc-rs</a> – DBC parser/decoder</li><li><a href="https://crates.io/crates/socketcan" target="_blank">socketcan</a> – CAN FD bindings</li></ul></div>
            <div class="cv-card"><div class="cv-card-header"><span class="cv-card-title">Rust Ecosystem</span></div><ul class="cv-card-body cv-deps-list"><li><a href="https://tokio.rs" target="_blank">Tokio</a> – Async runtime</li><li><a href="https://serde.rs" target="_blank">Serde</a> – Serialization</li><li><a href="https://clap.rs" target="_blank">Clap</a> – CLI parser</li></ul></div>
            <div class="cv-card"><div class="cv-card-header"><span class="cv-card-title">Frontend</span></div><ul class="cv-card-body cv-deps-list"><li><a href="https://vite.dev" target="_blank">Vite</a> – Build tool</li><li><a href="https://www.typescriptlang.org" target="_blank">TypeScript</a> – Typed JavaScript</li><li><a href="https://vitest.dev" target="_blank">Vitest</a> – Test framework</li></ul></div>
          </div>
          <p class="cv-about-license">MIT or Apache-2.0 • Rust + TypeScript</p>
        </div>
        ${t}
      </section>
    `}cacheElements(){this.mdf4Inspector=this.shadow.querySelector("cv-mdf4-inspector"),this.liveViewer=this.shadow.querySelector("cv-live-viewer"),this.dbcEditor=this.shadow.querySelector("cv-dbc-editor")}bindEvents(){this.shadow.querySelectorAll(".cv-tabs.bordered .cv-tab").forEach(e=>{e.addEventListener("click",()=>{const t=e.dataset.tab;t&&this.switchTab(t)})}),this.shadow.querySelector("cv-mdf4-toolbar")?.addEventListener("open",()=>this.mdf4Inspector?.promptLoadMdf4()),this.shadow.querySelector("cv-mdf4-toolbar")?.addEventListener("clear",()=>this.mdf4Inspector?.clearAllData()),this.shadow.querySelector("cv-live-toolbar")?.addEventListener("refresh-interfaces",()=>this.liveViewer?.loadInterfaces()),this.shadow.querySelector("cv-live-toolbar")?.addEventListener("start-capture",e=>{const t=e.detail.interface;this.liveViewer?.startCapture(t)}),this.shadow.querySelector("cv-live-toolbar")?.addEventListener("stop-capture",()=>this.liveViewer?.stopCapture()),this.shadow.querySelector("cv-live-toolbar")?.addEventListener("clear",()=>this.liveViewer?.clearAllData()),this.shadow.querySelector("cv-dbc-toolbar")?.addEventListener("new",()=>this.dbcEditor?.handleNew()),this.shadow.querySelector("cv-dbc-toolbar")?.addEventListener("open",()=>this.dbcEditor?.handleOpen()),this.shadow.querySelector("cv-dbc-toolbar")?.addEventListener("edit",()=>this.dbcEditor?.setEditMode(!0)),this.shadow.querySelector("cv-dbc-toolbar")?.addEventListener("cancel",()=>this.dbcEditor?.cancelEdit()),this.shadow.querySelector("cv-dbc-toolbar")?.addEventListener("save",()=>this.dbcEditor?.handleSave()),this.shadow.querySelector("cv-dbc-toolbar")?.addEventListener("save-as",()=>this.dbcEditor?.handleSaveAs()),this.shadow.querySelector("#aboutPanel")?.querySelectorAll(".cv-tab").forEach(e=>{e.addEventListener("click",()=>{const t=e.dataset.tab;t&&(this.shadow.querySelector("#aboutPanel")?.querySelectorAll(".cv-tab").forEach(s=>s.classList.toggle("active",s.dataset.tab===t)),this.shadow.querySelector("#aboutPanel")?.querySelectorAll(".cv-tab-pane").forEach(s=>s.classList.toggle("active",s.id===`about${t.charAt(0).toUpperCase()+t.slice(1)}`)))})}),this.shadow.addEventListener("click",e=>{const s=e.target.closest("a[href]");s?.href&&s.target==="_blank"&&(e.preventDefault(),this.openExternalUrl(s.href))})}setupComponents(){this.api&&(this.mdf4Inspector&&this.mdf4Inspector.setApi(this.createMdf4Api()),this.liveViewer&&this.liveViewer.setApi(this.createLiveApi()),this.dbcEditor&&this.dbcEditor.setApi(this.createDbcEditorApi()))}createMdf4Api(){const e=this.api;return{loadMdf4:t=>e.loadMdf4(t),decodeFrames:t=>e.decodeFrames(t),openFileDialog:t=>e.openFileDialog(t),getDbcInfo:()=>e.getDbcInfo()}}createLiveApi(){const e=this.api;return{listCanInterfaces:()=>e.listCanInterfaces(),startCapture:(t,s,a)=>e.startCapture(t,s,a),stopCapture:()=>e.stopCapture(),saveFileDialog:(t,s)=>e.saveFileDialog(t,s),getDbcInfo:()=>e.getDbcInfo(),onLiveCaptureUpdate:t=>e.onLiveCaptureUpdate(t),onCaptureFinalized:t=>e.onCaptureFinalized(t),onCaptureError:t=>e.onCaptureError(t)}}createDbcEditorApi(){const e=this.api;return{loadDbc:async t=>{await e.loadDbc(t);const s=await e.getDbcInfo();if(!s)throw new Error("Failed to load DBC");return this.state.dbcLoaded=!0,this.state.dbcFilename=$(t),k.set({dbcFile:t}),this.emitDbcChange("loaded",s),Se(s)},saveDbcContent:async(t,s)=>{await e.saveDbcContent(t,s),this.state.dbcFilename=$(t),k.set({dbcFile:t});const a=await e.getDbcInfo();this.emitDbcChange("updated",a)},newDbc:async()=>(await e.clearDbc(),this.state.dbcLoaded=!1,this.state.dbcFilename=null,k.set({dbcFile:null}),this.emitDbcChange("new",null),Et()),getDbc:async()=>{try{const t=await e.getDbcInfo();return t?Se(t):null}catch{return null}},updateDbc:async t=>{const s=se(t);await e.updateDbcContent(s),this.state.dbcLoaded=!0;const a=await e.getDbcInfo();this.emitDbcChange("updated",a)},getCurrentFile:async()=>e.getDbcPath(),isDirty:async()=>!1,openFileDialog:async()=>e.openFileDialog([{name:"DBC Files",extensions:["dbc"]}]),saveFileDialog:async()=>e.saveFileDialog([{name:"DBC Files",extensions:["dbc"]}],"untitled.dbc"),getDbcSpecification:async()=>e.getDbcSpecification()}}emitDbcChange(e,t){O({action:e,dbcInfo:t,filename:this.state.dbcFilename})}switchTab(e){if(this.state.activeTab==="dbc"&&e!=="dbc"&&this.dbcEditor?.hasUnsavedChanges()&&!confirm("You have unsaved changes in the DBC editor. Leave anyway?"))return;this.state.activeTab=e,localStorage.setItem(Ae,e),this.shadow.querySelectorAll(".cv-tabs.bordered .cv-tab").forEach(r=>{r.classList.toggle("active",r.dataset.tab===e)}),this.shadow.querySelectorAll(".cv-app-header > .cv-tab-pane").forEach(r=>{r.classList.toggle("active",r.id===`${e}Tab`)});const t=this.shadow.querySelector("#mdf4Panel"),s=this.shadow.querySelector("#livePanel"),a=this.shadow.querySelector("#dbcPanel"),n=this.shadow.querySelector("#aboutPanel");t?.classList.toggle("hidden",e!=="mdf4"),s?.classList.toggle("hidden",e!=="live"),a?.classList.toggle("hidden",e!=="dbc"),n?.classList.toggle("hidden",e!=="about");for(const r of this.extensions){const o=r.tab?.id||r.id;this.shadow.querySelector(`#${o}Panel`)?.classList.toggle("hidden",e!==o)}}async loadInitialFiles(){if(this.api)try{const e=await this.api.getInitialFiles();e.dbc_path&&this.dbcEditor&&(await this.dbcEditor.loadFile(e.dbc_path),this.switchTab("mdf4")),e.mdf4_path&&this.mdf4Inspector&&(await this.mdf4Inspector.loadFile(e.mdf4_path),this.switchTab("mdf4"))}catch(e){console.error("Failed to load initial files:",e)}}async openExternalUrl(e){try{const{open:t}=await at(async()=>{const{open:s}=await import("./chunk-Dhbb3tL-.js");return{open:s}},[]);await t(e)}catch{window.open(e,"_blank")}}}customElements.define("can-viewer",Fs);async function Re(){const i=new et;await i.init();const e=document.querySelector("can-viewer");e&&e.setApi(i)}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",Re):Re();export{re as i};
