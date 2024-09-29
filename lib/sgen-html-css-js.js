/*version: 0.1.4*/
const y=["readonly"],I=["html","body","div","span","p","textarea","field"],G=["0","1","2","3","4","5","6","7","8","9"];regexp("\\p{L}"),regexp("\\p{N}");function U(r){return H(G,r[0])}function $(r){if(U(r))return r;{let e="",t=0;for(;t<r.length;){switch(r[t]){case"\u0660":e+="0";break;case"\u0661":e+="1";break;case"\u0662":e+="2";break;case"\u0663":e+="3";break;case"\u0664":e+="4";break;case"\u0665":e+="5";break;case"\u0666":e+="6";break;case"\u0667":e+="7";break;case"\u0668":e+="8";break;case"\u0669":e+="9";break;case",":e+=".";break;default:h()}t+=1}return e}}function E(r){return r instanceof Array}function H(r,e){return r.includes(e)}function l(r,e){let t=[];const s=(i,n)=>{if(n&&typeof n=="object"){if(t.includes(n))return"[CIRCULAR]";t.push(n)}return n};return e?JSON.stringify(r,s,e):JSON.stringify(r,s)}function h(r){throw new Error(r)}function j(r,e,t){switch(r.id){case"call":let s=r.v[0].v.v[1],i=r.v[1]||[],n=r.v[2]||[];if(s==="br")return e+="<br>";switch(s){case"select":e=P(i,e)+"} ";break;case"font_face":e=q(i,e);break;case"keyframes":e=D(i,n,e);break;default:e+=`<${s}`,i.forEach((a,c)=>{if(c===0&&a.id==="str")e+=` id='${a.v.v[1]}'`;else if(a.id==="named_arg"){const p=a.v[0].v[1];if(y.includes(p))e+=` ${p} `;else if(e+=` ${p}= `,a.v[1].id==="str")e+=`'${a.v[1].v.v[1]}'`;else if(a.v[1].id==="int"||a.v[1].id==="float"){const o=a.v[1].v[0].v[1],_=a.v[1].v[1].v[1]||"";e+=`${o}${_}`}else a.v[1].id==="bool"?e+=`${p}`:h("not supported: "+l(a))}else h("not supported: "+l(a))}),e+=">",n.forEach(a=>{e=j(a,e)}),e+=`</${s}>`}break;case"str":e+=r.v.v[1];break;default:h("unknown html element: "+l(r))}return e}function P(r,e){return r.forEach(t=>{const s=f(t.v[0].v[1]),i=t.v[1];s==="element"?(e=W(i,e),e+=" {"):(e+=`${s} : `,e=d(i,e),e+="; ")}),e}function W(r,e){return E(r.v)?(e+=" ",r.v.forEach((t,s)=>{e+=`${t.v.v[1]} `,s<r.v.length-1&&(e+=",")})):e+=` ${r.v.v[1]}`,e}function d(r,e){switch(r.id){case"int":case"float":const t=r.v[0].v[1],s=r.v[1]&&r.v[1].v[1]||"";e+=t+s;break;case"prefix":e+=r.v.op.v,e=d(r.v.opr,e);break;case"postfix":e=d(r.v.opr,e),e+=r.v.op.v;break;case"str":e+=r.v.v[1];break;case"ref":e+=f(r.v.v[1]);break;case"tuple":r.v.forEach(p=>{e=d(p,e+" ")});break;case"call":const i=r.v[0].v.v[1],n=r.v[1];e+=` ${i}(`,n.forEach(p=>{e=d(p,e)}),e+=")";break;case"bin":const a=new JSGen;a.init(),a.write_expr(r);const c=a.get_code();pprint(c),e+="${"+c+"}".trim();break;default:h(`not supported: html generations:  ${l(r)}`)}return e}function q(r,e){return e+="@font-face { ",e=write_ar_css(r,e),e+="}",e}function D(r,e,t){return t+=` @keyframes ${r[0].v.v[1]} { `,e&&e.forEach(s=>{const i=s.v[0].v.v[1],n=s.v[1];switch(i){case"at":const a=n[0],c=n[1].v||[];t=d(a,t),t+=" {",c.forEach(p=>{if(p.id==="named_tuple")p.v.forEach(o=>{const _=f(o[0].v[1]),u=o[1];t+=` ${_} : `,t=d(u,t),t+="; "});else{const o=f(p[0].v[1]),_=p[1];t+=` ${o} : `,t=d(_,t),t+="; "}}),t+="} ";break;default:h("unsupported element: "+l(i))}}),t+="}",t}const g={\u0635\u0641\u062D\u0629_\u0627\u0644\u0634\u0628\u0643\u0629:"html",\u0631\u0627\u0633:"head",\u0646\u0633\u0642:"style",\u0645\u062A\u0646:"body",\u0645\u0646\u0637\u0642\u0629_\u0627\u0644\u0646\u0635:"textarea",\u0639\u0646\u0648\u0627\u0646_\u0631\u0627\u0633\u064A\u0663:"h3",\u0642\u0633\u0645:"div",\u0633\u0637\u0631:"br"},S=r=>{switch(r){case"\u0635\u0646\u0641":return"class";case"\u0627\u0639\u0645\u062F\u0629":return"cols";case"\u0635\u0641\u0648\u0641":return"rows";case"\u0644\u0644\u0642\u0631\u0627\u0621\u0629_\u0641\u0642\u0637":return"readonly";default:return r}},C={\u062D\u0648\u0645:"hover"},T=r=>{switch(r){case"\u0639\u0646\u062F":return"at";case"\u0627\u0632\u0627\u062D\u0629_\u0633":return"translateX";case"\u0639\u0646\u0648\u0627\u0646":return"url";default:return r}},A=r=>{switch(r){case"\u0639_\u0635":return"px";case"\u0639_\u0637":return"vh";case"\u0645_\u062C":return"rem";case"\u062B":return"s";case"\u066A":return"%";default:return r}},x=r=>{switch(r){case"\u0639\u0646\u0635\u0631":return"element";case"\u0639\u0631\u0636":return"width";case"\u0627\u062F\u0646\u0649_\u0639\u0631\u0636":return"min_width";case"\u0627\u0631\u062A\u0641\u0627\u0639":return"height";case"\u0627\u062F\u0646\u0649_\u0627\u0631\u062A\u0641\u0627\u0639":return"min_height";case"\u0644\u0648\u0646":return"color";case"\u0627\u062A\u062C\u0627\u0647":return"direction";case"\u062E\u0644\u0641\u064A\u0629":return"background";case"\u0644\u0648\u0646_\u0627\u0644\u062E\u0644\u0641\u064A\u0629":return"background_color";case"\u0635\u0648\u0631\u0629_\u0627\u0644\u062E\u0644\u0641\u064A\u0629":return"background_image";case"\u0645\u0648\u0642\u0639_\u0627\u0644\u062E\u0644\u0641\u064A\u0629":return"background_position";case"\u062A\u0643\u0631\u0627\u0631_\u0627\u0644\u062E\u0644\u0641\u064A\u0629":return"background_repeat";case"\u0627\u0631\u0641\u0627\u0642_\u0627\u0644\u062E\u0644\u0641\u064A\u0629":return"background_attachment";case"\u0645\u0644\u0627\u0626\u0645\u0629_\u0627\u0644\u0639\u0646\u0635\u0631":return"object_fit";case"\u062D\u062C\u0645_\u0627\u0644\u062E\u0644\u0641\u064A\u0629":return"background_size";case"_\u0622\u0628\u0644_\u062D\u062C\u0645_\u0627\u0644\u062E\u0644\u0641\u064A\u0629":return"_webkite_background_size";case"\u0641\u0627\u0626\u0636":return"overflow";case"\u0639\u062A\u0627\u0645\u0629":return"opacity";case"\u0627\u0638\u0647\u0627\u0631":return"display";case"\u0647\u0627\u0645\u0634":return"margin";case"\u0647\u0627\u0645\u0634_\u0639\u0644\u0648\u064A":return"margin_top";case"\u0647\u0627\u0645\u0634_\u0633\u0641\u0644\u064A":return"margin_bottom";case"\u0647\u0627\u0645\u0634_\u0627\u064A\u0645\u0646":return"margin_right";case"\u0647\u0627\u0645\u0634_\u0627\u064A\u0633\u0631":return"margin_left";case"\u0628\u0637\u0627\u0646\u0629":return"padding";case"\u062A\u062D\u062C\u064A\u0645_\u0627\u0644\u0635\u0646\u062F\u0648\u0642":return"box_sizing";case"\u0636\u0628\u0637_\u0627\u0644\u0645\u062D\u062A\u0648\u0649":return"justify_content";case"\u0636\u0628\u0637_\u0627\u0644\u0639\u0646\u0627\u0635\u0631":return"justify_items";case"\u0636\u0628\u0637_\u0627\u0644\u0646\u0635":return"text_justify";case"\u0645\u062D\u0627\u0630\u0627\u0629_\u0627\u0644\u0639\u0646\u0627\u0635\u0631":return"align_items";case"\u0645\u062D\u0627\u0630\u0627\u0629_\u0627\u0644\u0646\u0635":return"text_align";case"\u062D\u062C\u0645_\u0627\u0644\u062E\u0637":return"font_size";case"\u0641\u0635\u064A\u0644\u0629_\u0627\u0644\u062E\u0637":return"font_family";case"\u0641\u062C\u0648\u0629":return"gap";case"\u062D\u062F\u0648\u062F":return"border";case"\u0642\u0637\u0631_\u0627\u0644\u062D\u062F\u0648\u062F":return"border_radius";case"\u0646\u0633\u0642_\u0627\u0644\u062D\u062F\u0648\u062F":return"border_style";case"\u062D\u062F\u0648\u062F_\u062E\u0627\u0631\u062C\u064A\u0629":return"outline";case"\u0645\u0648\u0636\u0639":return"position";case"\u062A\u062D\u0631\u064A\u0643":return"animation";case"\u062A\u062D\u0648\u0644":return"transform";case"\u0627\u0639\u0627\u062F\u0629_\u062A\u062D\u062C\u064A\u0645":return"resize";case"\u0645\u0635\u062F\u0631":return"src";case"\u0646\u0633\u0628\u0629_\u0633_\u0635":return"aspect_ratio";case"\u0645\u0631\u0646_\u0628\u0627\u062A\u062C\u0627\u0647":return"flex_direction";case"\u0634\u0631\u064A\u0637_\u0627\u0644\u062A\u0645\u0631\u064A\u0631_\u0639\u0631\u0636":return"scrollbar_width";case"\u0642\u062F\u0631\u0629_\u0627\u062E\u062A\u064A\u0627\u0631_\u0627\u0644\u0646\u0635":return"user_select";case"_\u0645\u0627\u064A\u0643\u0631\u0648\u0633\u0648\u0641\u062A_\u0642\u062F\u0631\u0629_\u0627\u062E\u062A\u064A\u0627\u0631_\u0627\u0644\u0646\u0635":return"_ms_user_select";case"_\u0622\u0628\u0644_\u0642\u062F\u0631\u0629_\u0627\u062E\u062A\u064A\u0627\u0631_\u0627\u0644\u0646\u0635":return"_webkit_user_select";case"_\u0645\u0648\u0632\u064A\u0644\u0627_\u0642\u062F\u0631\u0629_\u0627\u062E\u062A\u064A\u0627\u0631_\u0627\u0644\u0646\u0635":return"_moz_user_select";case"\u062E\u064A\u0627\u0644_\u0627\u0644\u0635\u0646\u062F\u0648\u0642":return"box_shadow";case"_\u0622\u0628\u0644_\u062E\u064A\u0627\u0644_\u0627\u0644\u0635\u0646\u062F\u0648\u0642":return"_webkit_box_shadow";case"_\u0645\u0648\u0632\u064A\u0644\u0627_\u062E\u064A\u0627\u0644_\u0627\u0644\u0635\u0646\u062F\u0648\u0642":return"_moz_box_shadow";default:return r}},k=r=>{switch(r){case"\u062A\u0644\u0642\u0627\u0626\u064A":return"auto";case"\u062D\u062F\u0648\u062F_\u0627\u0644\u0635\u0646\u062F\u0648\u0642":return"border_box";case"\u0628\u0644\u0627_\u0642\u064A\u0645\u0629":return"none";case"\u0645\u0637\u0644\u0642":return"absolute";case"\u0645\u0631\u0646":return"flex";case"\u0645\u062E\u0641\u064A":return"hidden";case"\u0645\u0631\u0643\u0632":return"center";case"\u0645\u0633\u0627\u0641\u0629_\u0628\u064A\u0646":return"space_between";case"\u0628\u062F\u0627\u064A\u0629":return"start";case"\u0646\u0647\u0627\u064A\u0629":return"end";case"\u0628\u0627\u0631\u0632":return"ridge";case"\u0644\u0627_\u0646\u0647\u0627\u064A\u0629":return"infinite";case"\u0644\u0627_\u062A\u0643\u0631\u0627\u0631":return"no_repeat";case"\u0627\u062D\u062A\u0648\u0627\u0621":return"contain";case"\u0642\u0637\u0639":return"clip";case"\u0636\u0639\u0641":return"double";case"\u0636\u0628\u0637":return"justify";case"\u0628\u064A\u0646_\u0627\u0644\u0643\u0644\u0645\u0627\u062A":return"inter_word";case"\u0645\u0647\u0645":return"important";case"\u063A\u064A\u0631_\u0645\u0647\u0645":return"!important";case"\u0645\u062B\u0628\u062A":return"fixed";case"\u0645\u0646_\u0627\u0644\u064A\u0645\u064A\u0646":return"rtl";case"\u0639\u0645\u0648\u062F\u064A":return"column";case"\u0627\u0641\u0642\u064A":return"row";case"\u0633\u0645\u0627\u0648\u064A_\u0641\u0627\u062A\u062D":return"lightskyblue";case"\u0627\u0628\u064A\u0636":return"white";case"\u0627\u0635\u0641\u0631":return"yellow";case"\u0627\u0633\u0648\u062F":return"black";case"\u0628\u0631\u062A\u0642\u0627\u0644\u064A":return"orange";default:return r}},N=r=>{switch(r){case"\u0645\u062A\u0646":return"body";case"\u0635\u0641\u062D\u0629_\u0627\u0644\u0634\u0628\u0643\u0629":return"html";default:return r}};function O(r,e,t){switch(r.id){case"call":const s=r.v[0].v.v[1],i=g[s]||s,n=r.v[1]||[],a=r.v[2]||[];if(i==="br")return e+="<br>";switch(i){case"\u0627\u062E\u062A\u0631":e=J(n,e)+"} ";break;case"\u0639\u0631\u0641_\u062E\u0637":e=Y(n,e);break;case"\u0627\u0637\u0627\u0631\u0627\u062A_\u0631\u0626\u064A\u0633\u064A\u0629":e=B(n,a,e);break;default:e+=`<${i}`,I.includes(i)&&(e+=" dir='rtl'"),n.forEach((c,p)=>{if(p===0&&c.id==="str")e+=` id='${c.v.v[1]}'`;else if(c.id==="named_arg"){const o=S(c.v[0].v[1]);if(y.includes(o))e+=` ${o} `;else if(e+=` ${o}= `,c.v[1].id==="str")e+=`'${c.v[1].v.v[1]}'`;else if(c.v[1].id==="int"||c.v[1].id==="float"){const _=$(c.v[1].v[0].v[1]),u=c.v[1].v[1]&&A(c.v[1].v[1].v[1])||"";e+=`${_}${u}`}else c.v[1].id==="bool"?e+=`${S(o)}`:h("not supported: "+l(c))}else h("not supported: "+l(c))}),e+=">",a.forEach(c=>{e=O(c,e)}),e+=`</${i}>`}break;case"str":e+=r.v.v[1];break;default:h("unknown html element: "+l(r))}return e}function J(r,e){return r.forEach(t=>{const s=f(x(t.v[0].v[1])),i=t.v[1];s==="element"?(e=X(i,e),e+=" {"):(e+=`${s} : `,e=v(i,e),e+="; ")}),e}function X(r,e){const t=s=>{const i=n=>RegExp(`(?<![p{L}\\p{N}_])${n}(?![\\p{L}\\p{N}_])`,"ug");return Object.keys(g).forEach(n=>{s=s.replaceAll(i(n),g[n])}),Object.keys(C).forEach(n=>{s=s.replaceAll(i(n),C[n])}),s};if(E(r.v))e+=" ",r.v.forEach((s,i)=>{let n=s.v.v[1];e+=t(n),i<r.v.length-1&&(e+=",")});else{const s=r.v.v[1];e+=t(s)}return e}function v(r,e){switch(r.id){case"bool":h();break;case"int":case"float":const t=$(r.v[0].v[1]),s=A(r.v[1]&&r.v[1].v[1])||"";e+=t+s;break;case"prefix":e+=r.v.op.v,e=v(r.v.opr,e);break;case"postfix":e=v(r.v.opr,e),e+=r.v.op.v;break;case"str":e+=N(r.v.v[1]);break;case"ref":e+=f(k(r.v.v[1]));break;case"tuple":r.v.forEach(c=>{e=v(c,e+" ")});break;case"call":const i=T(r.v[0].v.v[1]),n=r.v[1];e+=` ${i}(`,n.forEach(c=>{e=v(c,e)}),e+=")";break;case"bin":jsGen.init(),jsGen.write_expr(r);const a=jsGen.get_code();e+="${"+a+"}".trim();break;default:h(`not supported: html generations:  ${l(r)}`)}return e}function Y(r,e){return e+="@font-face { ",e=J(r,e),e+="}",e}function B(r,e,t){return t+=` @keyframes ${r[0].v.v[1]} { `,e&&e.forEach(s=>{const i=s.v[0].v.v[1],n=k(s.v[1]);switch(i){case"\u0639\u0646\u062F":const a=n[0],c=n[1].v||[];t=v(a,t),t+=" {",c.forEach(p=>{if(p.id==="named_tuple")p.v.forEach(o=>{const _=f(x(o[0].v[1])),u=k(o[1]);t+=` ${_} : `,t=v(u,t),t+="; "});else{const o=f(x(p[0].v[1])),_=k(p[1]);t+=` ${o} : `,t=v(_,t),t+="; "}}),t+="} ";break;default:h("unsupported element: "+l(i))}}),t+="}",t}class F{write_html;write_ar_html;CSS_value_en;CSS_str_en;constructor(){this.write_html=j,this.write_ar_html=O,this.CSS_value_en=k,this.CSS_str_en=N}}function f(r){return HYPHENATED.includes(r)?r.replaceAll("_","-"):r}const K=["0","1","2","3","4","5","6","7","8","9"];regexp("\\p{L}"),regexp("\\p{N}");function M(r){return R(K,r[0])}function L(r){if(M(r))return r;{let e="",t=0;for(;t<r.length;){switch(r[t]){case"\u0660":e+="0";break;case"\u0661":e+="1";break;case"\u0662":e+="2";break;case"\u0663":e+="3";break;case"\u0664":e+="4";break;case"\u0665":e+="5";break;case"\u0666":e+="6";break;case"\u0667":e+="7";break;case"\u0668":e+="8";break;case"\u0669":e+="9";break;case",":e+=".";break;default:w()}t+=1}return e}}function Q(r){return r instanceof Array}function R(r,e){return r.includes(e)}function b(r,e){let t=[];const s=(i,n)=>{if(n&&typeof n=="object"){if(t.includes(n))return"[CIRCULAR]";t.push(n)}return n};return e?JSON.stringify(r,s,e):JSON.stringify(r,s)}function w(r){throw new Error(r)}function V(r,e){return r.repeat(e)}const Z=`
//------------------------------------------------------------------------------
// js helper functions injected to workaround missing seen features that are yet to be added.
function is_none(x) { return x == null }        
function is_list(x) { return x instanceof Array }
function replace(array, i, v) {  array[i] = v }
function to_int(str) { return parseInt(str) }
function assign(x,y) { x = y }
function concat(x,y,id) { x[id] = x[id].concat(y[id]) }
function del(array, i) { delete array[i] }
function regexp(expr) { return RegExp(expr, 'ug') }
function match_regexp(v, expr) {return expr.exec(v) }
function print(v) { throw new Error('print() is not implemeted')}
function \u0627\u0637\u0628\u0639_\u0633\u0637\u0631(v) { println(v) }
function println(v) {         
    if(v == null ) { console.log("undefined") } else { console.log(v) }
}
function panic(v) { throw new Error(v)}
function clone(obj) { return {...obj} }
function contains(list, el) { return list.includes(el) }
function is_empty(list) { return Array.isArray(list) && list.length === 0 }
function \u0627\u0637\u0628\u0639_\u062A\u0641\u0627\u0635\u064A\u0644(obj, indent) { pprint(obj, indent) }
function pprint(obj, indent) { 
    if( obj == null ) {
        console.log("undefined")
    } else {
        if(indent) {
            console.log(JSON.stringify(obj, null, indent)) 
        } else {
            console.log(JSON.stringify(obj)) 
        }       
    }
}
function to_str(obj, indent) { 
let objects = []
function eliminateCircular(k, v) {
    if (v && typeof v === 'object') {
        if (objects.includes(v)) { return "[CIRCULAR]" } else { objects.push(v) }
    }
    return v
}
if(indent) {
    return JSON.stringify(obj, eliminateCircular, indent)
} else {
    return JSON.stringify(obj, eliminateCircular)
}
}
function repeat(str, times) { return str.repeat(times) }
function c0_to_uppercase(str){ return str.charAt(0).toUpperCase() + str.slice(1) }
function to_lowercase(str) {return str.toLowerCase()}
function \u0639\u0631\u0636_\u0627\u0648\u0644\u064A(code, preview_id){ preview(code, preview_id) }
function preview(code, preview_id) { window.parent.document.querySelector(preview_id).srcdoc = code }
function \u0647\u0627\u062A_\u0627\u0644\u0627\u0641\u0631\u0639(\u0633) {
    return \u0633.__children
}
function \u0627\u062E\u062A\u0631(\u0633,\u062F\u0627\u0644\u0629) {
    return \u0633.filter(\u062F\u0627\u0644\u0629)
}
function \u0647\u0627\u062A(\u0642,\u0641\u0647\u0631\u0633) { return \u0642[\u0641\u0647\u0631\u0633]}
function \u0639\u062F\u062F_\u0627\u0644\u0639\u0646\u0627\u0635\u0631(\u0642) { return \u0642.length}
async function read_url(url) {
    const response = await fetch(url);
    return response.text()
}

//------------------------------------------------------------------------------
`,z={\u0628\u062F\u0621:"main",\u0627\u0637\u0628\u0639_\u0633\u0637\u0631:"println",\u062A\u0639\u0628\u064A\u0631_\u0646\u0645\u0637\u064A:"regex",\u0647\u0630\u0627:"this",\u0645\u0634\u064A\u0651\u062F:"constructor",\u0627\u0646\u0647\u0627\u0621:"panic"},ee=4;class m{current;indent_level;stack;astructs;ast;symtab;html_gen;main_args;opts;constructor(e,t,s,i,n,a,c,p){return this.current=e,this.indent_level=t,this.stack=s,this.astructs=i,this.ast=n,this.symtab=a,this.html_gen=c,this.opts=p,this}init(e,t,s,i,n){this.current="",this.indent_level=0,this.stack=[],this.astructs=[],this.ast=e,this.symtab=t,this.html_gen=s,this.main_args=i,this.opts=n}run(){this.strict_mode();let e,t=0;for(;t<this.ast.length;){const s=this.ast[t];if(s){const i=s.v;switch(s.id){case"use":this.write_use(i);break;case"modif":this.write_modifier(i);break;case"main":e=i;break;case"const":this.write_const(i);break;case"fn":this.write_fn(i);break;case"type":this.write_typedef(i);break;default:w("unsupported node: "+this.ast[t].id)}}t+=1}return this.write_helper_fns(),e&&this.write_main(e),this.get_code()}to_en_id(e){!e.v&&!Q(e.v)||z[e.v[1]]&&(e.v[1]=z[e.v[1]])}push(){this.stack.push(this.current),this.current=""}pop(){this.current=this.stack.pop()+this.current}pop_prepend(){this.current=this.current+this.stack.pop()}append(e){this.current+=e}appendi(e){this.current+=this.spaces(),this.current+=e}spaces(e){return e||(e=this.indent_level),V(" ",e*ee)}strict_mode(){this.append(`"use strict";

`)}write_id_pat(e){const t=e.v.v[1];this.append(t==="_"?"default":t)}write_char_pat(e){this.append("'"+e.v.v[1]+"'")}write_str_pat(e){this.append('"'+e.v.v[1]+'"')}write_tuple_pat(e){this.append("(");let t=0;for(;t<e.v.length;)this.write_pat(e.v[t]),t<e.v.length-1&&this.append(", "),t+=1;this.append(")")}write_pat(e){switch(e.id){case"id":this.write_id_pat(e);break;case"bool":this.append(e.v.v[1]);break;case"int":case"float":this.append(L(e.v.v[1][0]));break;case"char":this.write_char_pat(e);break;case"str":this.write_str_pat(e);break;case"tuple":this.write_tuple_pat(e);break;case"_":this.append("default");break;default:w("unsupported pattern "+b(e))}}write_modifier(e){this.opts.ignore_export||e.v==="+"&&this.appendi("export ")}write_use(e){}write_main(e){this.push(),this.appendi("("),this.write_fn(e,this.main_args),this.appendi(`)()
`),this.pop()}write_params(e){this.append("(");let t=0;for(;t<e.length;)t>0&&this.append(", "),this.write_pat(e[t].v._pat),t+=1;this.append(")")}write_do_block(e){this.append("(()=>"),this.write_block(e),this.append(`)() 
`)}write_block(e){this.append(` {
`),this.push(),this.indent_level+=1;let t=0;const s=e.v.length;for(;t<s;){const i=e.v[t];this.write_stmt(i),t+=1}this.pop(),this.indent_level-=1,this.appendi(`}
`)}write_fn(e,t){this.push(),e.t==="fn"&&this.appendi("static "),this.to_en_id(e.name),this.append("function "+e.name.v[1]),t?this.append("()"):this.write_params(e.params),this.write_body(e.body,e.name==="main",t),this.pop()}write_fields(e){const t=[];e.forEach(s=>{const i=s.v[0].v[1];t.push(i)}),t.forEach(s=>{this.appendi(this.spaces()+""+s+`
`)}),this.write_init(t)}write_init(e){this.append(`
`),this.appendi("constructor(");let t=0;for(;t<e.length;)this.append(e[t]),t<e.length-1&&this.append(", "),t+=1;for(this.append(`) {
`),this.indent_level+=1,t=0;t<e.length;)this.appendi("this."+e[t]+" = "+e[t]+`
`),t+=1;this.appendi(`return this
`),this.indent_level-=1,this.appendi(`}
`)}write_typedef(e){this.appendi("class "+e.name.v[1]+` {
`),this.indent_level+=1,e.fields&&this.write_fields(e.fields),this.indent_level-=1,this.appendi(`}

`)}write_const(e){this.appendi("const "),this.write_pat(e.lhs),this.append(" = "),this.write_expr(e.rhs),this.append(`
`)}write_var(e){this.appendi("let "),this.write_pat(e.lhs),e.rhs&&(this.append(" = "),this.write_expr(e.rhs)),this.append(`
`)}write_ret(e){this.append("return "),e.v&&this.write_expr(e.v)}write_break(e){this.append("break")}write_stmt(e){e.t==="expr"?(this.appendi(""),this.write_expr(e),this.append(`
`)):e.id==="const"?this.write_const(e.v):e.id==="var"?this.write_var(e.v):e.id==="break"?this.write_break(e):w("cannot write stmt: "+b(e))}write_body(e,t,s){if(this.append(` {
`),this.push(),this.indent_level+=1,s)for(const[a,c]of Object.entries(s))this.append(`const ${a} = '${c}'
`);let i=0;const n=e.v.length;for(;i<n;){const a=e.v[i];this.write_stmt(a),i+=1}this.pop(),this.indent_level-=1,this.appendi(`}
`)}write_id(e){this.append(e.v[1])}write_ref(e){const t=e.v.v[1];this.append(t)}write_str(e){const t=e.v.v[1],s=t.indexOf("${")===-1?'"':"`";this.append(s+t+s)}write_str_id(e){this.append(symbol+e.v.v[1]+symbol)}is_call(e){return e.v.id==="bin"&&e.v.v.op.v==="("}write_iret(e){R(["when","while","if","for","return"],e.v.node)||e.v.t==="()"||this.is_call(e)||e.semicolon||this.append("return "),this.is_call(e)?(this.append("const temp_seen_var = "),this.write_expr(e.v),this.append(`
`),this.append("return temp_seen_var")):this.write_expr(e.v)}write_list(e){this.append("[");let t=0;const s=e.v.length;for(;t<s;)this.write_expr(e.v[t]),t<e.v.length-1&&this.append(", "),t+=1;this.append("]")}write_structl(e){const t=e.v;this.append("{");let s=0;for(;s<t.length;){const i=t[s],n=i.k;n.v.v[1]?this.write_id(n.v):this.write_str_id(n.v);const a=i.v;this.append(": "),this.write_expr(a),s<t.length-1&&this.append(", "),s+=1}this.append("}")}write_args(e){this.append("(");let t=0;for(;t<e.v.length;){let s=e.v[t];s.v.op&&s.v.op.v===":"&&(s=s.v.ropr),this.write_expr(s),t<e.v.length-1&&this.append(", "),t+=1}this.append(")")}write_named_arg(e){e.v[0].v[1];const t=e.v[1];this.write_expr(t)}write_tuple(e){this.append("[");let t=0;for(;t<e.v.length;){let s=e.v[t];s.id==="narg"&&(s=e.v[t].v[1]),this.write_expr(s),t<e.v.length-1&&this.append(", "),t+=1}this.append("]")}write_named_tuple(e){this.append("{"),e.v.forEach((t,s)=>{const i=t[0].v[1],n=t[1];this.append(i),this.append(": "),this.write_expr(n),s<e.v.length&&this.append(",")}),this.append("}")}write_when(e){this.appendi("switch("),this.write_expr(e.v.expr),this.append(`) {
`),this.indent_level+=1;let t=0;for(;t<e.v.arms.length;){const s=e.v.arms[t],i=s.v.pats,n=s.v.expr;let a=0;for(;a<i.length;)i[a].id!=="_"&&this.appendi("case "),this.write_pat(i[a]),this.append(` :
`),a+=1;this.indent_level+=1,this.appendi(""),this.write_expr(n),this.append(`
`),this.appendi(`break
`),this.indent_level-=1,t+=1}this.indent_level-=1,this.appendi(`}
`)}write_prefix_uni(e){const t=e.v.op.v;switch(t){case".":if(e.v.opr.v.v[1]==="none"){this.append("null");return}else w("enum variants are not supported, found : (."+e.v.opr.v.v[1]+")");break;case"not":this.append("!");break;case"!":case"-":this.append(t);break;default:w("unsupported op: "+t);break}this.write_expr(e.v.opr)}write_pipe(e){for(;e.length>0;){let t=e.pop();switch(t.id){case"ref":this.write_expr(t),e.length>0&&(this.append("("),this.write_pipe(e),this.append(")"));break;case"call":const s=t.v[0],i=t.v[1];this.write_expr(s),this.append("("),this.write_pipe(e),this.current.slice(-1)!=="("&&i.length>0&&this.append(", "),i.forEach((n,a)=>{this.write_expr(n),a<i.length-1&&this.append(", ")}),this.append(")");break;case"int":case"float":case"str":case"[":case"tuple":case"named_tuple":this.write_expr(t);break;default:throw new Error("syntax error |> :"+b(t))}}}write_call(e){if(this.to_en_id(e.v[0].v),e.v[0].v.v[1]==="html"){const s=html_gen.write_html(e,"",m());this.append(` (() => \`${s}\`)() `);return}else if(e.v[0].v.v[1]==="\u0635\u0641\u062D\u0629_\u0627\u0644\u0634\u0628\u0643\u0629"){const s=html_gen.write_ar_html(e,"",m());this.append(`(() => \`${s}\`)()`);return}else symtab.structs.includes(e.v[0].v.v[1])&&this.append("new ");this.write_expr(e.v[0]),this.append("(");const t=e.v[1];t&&t.forEach((s,i)=>{this.write_expr(s),i<t.length-1&&this.append(", ")}),e.v[2]&&(t&&this.append(", "),this.write_children(e.v[2])),this.append(")")}write_children(e){!e||e.length===0||(this.append("["),e.forEach(t=>{this.write_expr(t),this.append(",")}),this.append("]"))}write_bin(e){const t=e.v.op.v;switch(t){case"[":this.write_expr(e.v.lopr),this.append("["),this.write_expr(e.v.ropr),this.append("]");break;case"=":this.write_expr(e.v.lopr),this.append("="),this.write_expr(e.v.ropr);break;case":=":this.append("let "),this.write_expr(e.v.lopr),this.append(" = "),this.write_expr(e.v.ropr),this.append(`
`);break;case"::":this.appendi("const "),this.write_expr(e.v.lopr),this.append(" = "),this.write_expr(e.v.ropr),this.append(`
`);break;case":":this.appendi("let "),this.write_expr(e.v.lopr),this.append(`
`);break;case"|>":{let s=[],i=e.v.lopr,n=e.v.ropr;for(;;)if(s.push(i),n.id==="bin"&&n.v.op.v==="|>")i=n.v.lopr,n=n.v.ropr;else{s.push(n);break}this.write_pipe(s);break}case"||>":throw new Error(" ||> : WIP , "+b(e));case":>":throw new Error(" :> : WIP , "+b(e));case"==":case"!=":case"<":case"<=":case">":case">=":case"|":case"||":case"&":case"&&":case"+":case"-":case"/":case"*":case"+=":case"-=":case"*=":case"\\=":case".":this.write_expr(e.v.lopr),this.append(t),(t==="=="||t==="!=")&&this.append("="),this.write_expr(e.v.ropr);break;default:w("cannot write binary operation: "+b(e));break}}write_afn(e){this.push(),this.write_params(e.v.params),this.append("=>"),this.write_body(e.v.body),this.pop()}write_expr(e){switch(e.grouped&&this.append("("),e.id){case"()":break;case";":break;case"ref":this.write_ref(e);break;case"bool":this.append(e.v.v[1]);break;case"int":case"float":this.append(L(e.v[0].v[1]));break;case"char":this.append("'"+e.v.v[1]+"'");break;case"str":this.write_str(e);break;case"return":this.write_ret(e);break;case"iret":this.write_iret(e);break;case"[":this.write_list(e);break;case"{":this.write_structl(e);break;case"args":this.write_args(e);break;case"named_arg":this.write_named_arg(e);break;case"tuple":this.write_tuple(e);break;case"named_tuple":this.write_named_tuple(e);break;case"when":this.write_when(e);break;case"do_block":this.write_do_block(e);break;case"block":this.write_block(e);break;case"prefix":this.write_prefix_uni(e);break;case"call":this.write_call(e);break;case"bin":this.write_bin(e);break;case"afn":this.write_afn(e);break;default:w("cannot write expr: "+b(e))}e.grouped&&this.append(")")}write_helper_fns(){this.append(Z)}get_code(){return this.current}}class te{run(e,t,s,i){const n=F(),a=m();return a.init(e,t,n,s,i),a.run()}}export{te as HtmlCssJSGen};