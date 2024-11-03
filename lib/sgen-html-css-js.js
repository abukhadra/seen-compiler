const HYPHENATED = [
  "min_width",
  "min_height",
  "background_color",
  "background_image",
  "background_position",
  "background_repeat",
  "background_size",
  "background_attachment",
  "_webkite_background_size",
  "aspect_ratio",
  "border_right",
  "border_left",
  "border_top",
  "border_bottom",
  "border_radius",
  "border_style",
  "margin_top",
  "margin_bottom",
  "margin_right",
  "margin_left",
  "align_items",
  "text_align",
  "justify_content",
  "justify_items",
  "text_justify",
  "object_fit",
  "font_size",
  "font_family",
  "box_sizing",
  "scrollbar_width",
  "user_select",
  "_ms_user_select",
  "_webkit_user_select",
  "_moz_user_select",
  "box_shadow",
  "_webkit_box_shadow",
  "_moz_box_shadow",
  "no_repeat",
  "border_box",
  "space_between",
  "flex_direction",
  "inter_word"
];
const BOOL_ATTRS = [
  "readonly"
];
const ELEMENTS_WITH_DIR = [
  "html",
  "body",
  "div",
  "span",
  "p",
  "textarea",
  "field"
];
const MAGHRIB_DIGIT$1 = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
function is_maghrib_num$1(n) {
  return contains$1(MAGHRIB_DIGIT$1, n[0]);
}
function to_maghrib_num$1(n) {
  if (!is_maghrib_num$1(n)) {
    let v = "";
    let i = 0;
    while (i < n.length) {
      const c = n[i];
      switch (c) {
        case "٠":
          v += "0";
          break;
        case "١":
          v += "1";
          break;
        case "٢":
          v += "2";
          break;
        case "٣":
          v += "3";
          break;
        case "٤":
          v += "4";
          break;
        case "٥":
          v += "5";
          break;
        case "٦":
          v += "6";
          break;
        case "٧":
          v += "7";
          break;
        case "٨":
          v += "8";
          break;
        case "٩":
          v += "9";
          break;
        case ",":
          v += ".";
          break;
        default:
          panic$1();
      }
      i += 1;
    }
    return v;
  } else {
    return n;
  }
}
function is_list$1(x) {
  return x instanceof Array;
}
function contains$1(list, el) {
  return list.includes(el);
}
function to_str$1(obj, indent) {
  let objects = [];
  const eliminateCircular = (k, v) => {
    if (v && typeof v === "object") {
      if (objects.includes(v)) {
        return "[CIRCULAR]";
      } else {
        objects.push(v);
      }
    }
    return v;
  };
  {
    return JSON.stringify(obj, eliminateCircular);
  }
}
function panic$1(v) {
  throw new Error(v);
}
class HtmlWriter {
  jsGen;
  constructor(jsGen) {
    jsGen.init();
    this.jsGen = jsGen;
    return this;
  }
  write_html(el, page) {
    switch (el.id) {
      case "call":
        let tag = el.v[0].v.v[1];
        let attrs = el.v[1] || [];
        let children = el.v[2] || [];
        if (tag === "br") {
          return page += "<br>";
        }
        switch (tag) {
          case "select":
            page = this.write_css(attrs, page) + "} ";
            break;
          case "font_face":
            page = this.write_css_fontface(attrs, page);
            break;
          case "keyframes":
            page = this.write_css_keyframes(attrs, children, page);
            break;
          default:
            page += `<${tag}`;
            attrs.forEach((attr, i) => {
              if (i === 0 && attr.id === "str") {
                page += ` id='${attr.v.v[1]}'`;
              } else if (attr.id === "named_arg") {
                const attr_name = attr.v[0].v[1];
                if (BOOL_ATTRS.includes(attr_name)) {
                  page += ` ${attr_name} `;
                } else {
                  page += ` ${attr_name}= `;
                  if (attr.v[1].id === "str") {
                    page += `'${attr.v[1].v.v[1]}'`;
                  } else if (attr.v[1].id === "int" || attr.v[1].id === "float") {
                    const num = attr.v[1].v[0].v[1];
                    const suffix = attr.v[1].v[1].v[1] || "";
                    page += `${num}${suffix}`;
                  } else if (attr.v[1].id === "bool") {
                    page += `${attr_name}`;
                  } else {
                    panic$1("not supported: " + to_str$1(attr));
                  }
                }
              } else {
                panic$1("not supported: " + to_str$1(attr));
              }
            });
            page += ">";
            children.forEach((c) => {
              page = this.write_html(c, page);
            });
            page += `</${tag}>`;
        }
        break;
      case "str":
        page += el.v.v[1];
        break;
      default:
        panic$1("unknown html element: " + to_str$1(el));
    }
    return page;
  }
  write_css(attrs, page) {
    attrs.forEach((attr) => {
      const k = maybe_hyphenated(attr.v[0].v[1]);
      const v = attr.v[1];
      if (k === "element") {
        page = this.write_css_selector(v, page);
        page += " {";
      } else {
        page += `${k} : `;
        page = this.write_css_attr_value(v, page);
        page += `; `;
      }
    });
    return page;
  }
  write_css_selector(v, page) {
    if (is_list$1(v.v)) {
      page += " ";
      v.v.forEach((x, i) => {
        page += `${x.v.v[1]} `;
        if (i < v.v.length - 1) {
          page += ",";
        }
      });
    } else {
      page += ` ${v.v.v[1]}`;
    }
    return page;
  }
  write_css_attr_value(v, page) {
    switch (v.id) {
      case "int":
      case "float":
        const num = v.v[0].v[1];
        const suffix = v.v[1] && v.v[1].v[1] || "";
        page += num + suffix;
        break;
      case "prefix":
        page += v.v.op.v;
        page = this.write_css_attr_value(v.v.opr, page);
        break;
      case "postfix":
        page = this.write_css_attr_value(v.v.opr, page);
        page += v.v.op.v;
        break;
      case "str":
        page += v.v.v[1];
        break;
      case "ref":
        page += maybe_hyphenated(v.v.v[1]);
        break;
      case "tuple":
        v.v.forEach((el) => {
          page = this.write_css_attr_value(el, page + " ");
        });
        break;
      case "call":
        const ref = v.v[0].v.v[1];
        const args = v.v[1];
        page += ` ${ref}(`;
        args.forEach((arg) => {
          page = this.write_css_attr_value(arg, page);
        });
        page += `)`;
        break;
      case "bin":
        this.jsGen.write_expr(v);
        const code = this.jsGen.get_code();
        page += "${" + code + "}".trim();
        break;
      default:
        panic$1(`not supported: html generations:  ${to_str$1(v)}`);
    }
    return page;
  }
  write_css_fontface(attrs, page) {
    page += `@font-face { `;
    page = write_ar_css(attrs, page);
    page += "}";
    return page;
  }
  write_css_keyframes(attrs, children, page) {
    page += ` @keyframes ${attrs[0].v.v[1]} { `;
    children && children.forEach((c) => {
      const ref = c.v[0].v.v[1];
      const v = c.v[1];
      switch (ref) {
        case "at":
          const percentage = v[0];
          const attrs2 = v[1].v || [];
          page = this.write_css_attr_value(percentage, page);
          page += " {";
          attrs2.forEach((attr) => {
            if (attr.id === "named_tuple") {
              attr.v.forEach((el) => {
                const _k = maybe_hyphenated(el[0].v[1]);
                const _v = el[1];
                page += ` ${_k} : `;
                page = this.write_css_attr_value(_v, page);
                page += `; `;
              });
            } else {
              const _k = maybe_hyphenated(attr[0].v[1]);
              const _v = attr[1];
              page += ` ${_k} : `;
              page = this.write_css_attr_value(_v, page);
              page += `; `;
            }
          });
          page += "} ";
          break;
        default:
          panic$1("unsupported element: " + to_str$1(ref));
      }
    });
    page += "}";
    return page;
  }
}
const HTML_tag_en = {
  "صفحة_الشبكة": "html",
  "راس": "head",
  "نسق": "style",
  "متن": "body",
  "منطقة_النص": "textarea",
  "عنوان_راسي٣": "h3",
  "قسم": "div",
  "سطر": "br"
};
const HTML_attr_en = (id) => {
  switch (id) {
    case "صنف":
      return "class";
    case "اعمدة":
      return "cols";
    case "صفوف":
      return "rows";
    case "للقراءة_فقط":
      return "readonly";
    default:
      return id;
  }
};
const CSS_pseudo_en = {
  // FIXME: workaround
  "حوم": "hover"
};
const CSS_fn_en = (id) => {
  switch (id) {
    case "عند":
      return "at";
    case "ازاحة_س":
      return "translateX";
    case "عنوان":
      return "url";
    default:
      return id;
  }
};
const CSS_suffix_en = (id) => {
  switch (id) {
    case "ع_ص":
      return "px";
    case "ع_ط":
      return "vh";
    case "م_ج":
      return "rem";
    case "ث":
      return "s";
    case "٪":
      return "%";
    default:
      return id;
  }
};
const CSS_key_en = (id) => {
  switch (id) {
    case "عنصر":
      return "element";
    case "عرض":
      return "width";
    case "ادنى_عرض":
      return "min_width";
    case "ارتفاع":
      return "height";
    case "ادنى_ارتفاع":
      return "min_height";
    case "لون":
      return "color";
    case "اتجاه":
      return "direction";
    case "خلفية":
      return "background";
    case "لون_الخلفية":
      return "background_color";
    case "صورة_الخلفية":
      return "background_image";
    case "موقع_الخلفية":
      return "background_position";
    case "تكرار_الخلفية":
      return "background_repeat";
    case "ارفاق_الخلفية":
      return "background_attachment";
    case "ملائمة_العنصر":
      return "object_fit";
    case "حجم_الخلفية":
      return "background_size";
    case "_آبل_حجم_الخلفية":
      return "_webkite_background_size";
    case "فائض":
      return "overflow";
    case "عتامة":
      return "opacity";
    case "اظهار":
      return "display";
    case "هامش":
      return "margin";
    case "هامش_علوي":
      return "margin_top";
    case "هامش_سفلي":
      return "margin_bottom";
    case "هامش_ايمن":
      return "margin_right";
    case "هامش_ايسر":
      return "margin_left";
    case "بطانة":
      return "padding";
    case "تحجيم_الصندوق":
      return "box_sizing";
    case "ضبط_المحتوى":
      return "justify_content";
    case "ضبط_العناصر":
      return "justify_items";
    case "ضبط_النص":
      return "text_justify";
    case "محاذاة_العناصر":
      return "align_items";
    case "محاذاة_النص":
      return "text_align";
    case "حجم_الخط":
      return "font_size";
    case "فصيلة_الخط":
      return "font_family";
    case "فجوة":
      return "gap";
    case "حدود":
      return "border";
    case "قطر_الحدود":
      return "border_radius";
    case "نسق_الحدود":
      return "border_style";
    case "حدود_خارجية":
      return "outline";
    case "موضع":
      return "position";
    case "تحريك":
      return "animation";
    case "تحول":
      return "transform";
    case "اعادة_تحجيم":
      return "resize";
    case "مصدر":
      return "src";
    case "نسبة_س_ص":
      return "aspect_ratio";
    case "مرن_باتجاه":
      return "flex_direction";
    case "شريط_التمرير_عرض":
      return "scrollbar_width";
    case "قدرة_اختيار_النص":
      return "user_select";
    case "_مايكروسوفت_قدرة_اختيار_النص":
      return "_ms_user_select";
    case "_آبل_قدرة_اختيار_النص":
      return "_webkit_user_select";
    case "_موزيلا_قدرة_اختيار_النص":
      return "_moz_user_select";
    case "خيال_الصندوق":
      return "box_shadow";
    case "_آبل_خيال_الصندوق":
      return "_webkit_box_shadow";
    case "_موزيلا_خيال_الصندوق":
      return "_moz_box_shadow";
    default:
      return id;
  }
};
const CSS_value_en = (id) => {
  switch (id) {
    case "تلقائي":
      return "auto";
    case "حدود_الصندوق":
      return "border_box";
    case "بلا_قيمة":
      return "none";
    case "مطلق":
      return "absolute";
    case "مرن":
      return "flex";
    case "مخفي":
      return "hidden";
    case "مركز":
      return "center";
    case "مسافة_بين":
      return "space_between";
    case "بداية":
      return "start";
    case "نهاية":
      return "end";
    case "بارز":
      return "ridge";
    case "لا_نهاية":
      return "infinite";
    case "لا_تكرار":
      return "no_repeat";
    case "احتواء":
      return "contain";
    case "قطع":
      return "clip";
    case "ضعف":
      return "double";
    case "ضبط":
      return "justify";
    case "بين_الكلمات":
      return "inter_word";
    case "مهم":
      return "important";
    case "غير_مهم":
      return "!important";
    case "مثبت":
      return "fixed";
    case "من_اليمين":
      return "rtl";
    case "عمودي":
      return "column";
    case "افقي":
      return "row";
    case "سماوي_فاتح":
      return "lightskyblue";
    case "ابيض":
      return "white";
    case "اصفر":
      return "yellow";
    case "اسود":
      return "black";
    case "برتقالي":
      return "orange";
    default:
      return id;
  }
};
const CSS_str_en = (id) => {
  switch (id) {
    case "متن":
      return "body";
    case "صفحة_الشبكة":
      return "html";
    default:
      return id;
  }
};
class ArHtmlWriter {
  jsGen;
  constructor(jsGen) {
    jsGen.init();
    this.jsGen = jsGen;
    return this;
  }
  write_ar_html(el, page) {
    switch (el.id) {
      case "call":
        const id = el.v[0].v.v[1];
        const tag = HTML_tag_en[id] || id;
        const attrs = el.v[1] || [];
        const children = el.v[2] || [];
        if (tag === "br") {
          return page += "<br>";
        }
        switch (tag) {
          case "اختر":
            page = this.write_ar_css(attrs, page) + "} ";
            break;
          case "عرف_خط":
            page = this.write_ar_css_fontface(attrs, page);
            break;
          case "اطارات_رئيسية":
            page = this.write_ar_css_keyframes(attrs, children, page);
            break;
          default:
            page += `<${tag}`;
            if (ELEMENTS_WITH_DIR.includes(tag)) {
              page += ` dir='rtl'`;
            }
            attrs.forEach((attr, i) => {
              if (i === 0 && attr.id === "str") {
                page += ` id='${attr.v.v[1]}'`;
              } else if (attr.id === "named_arg") {
                const attr_name = HTML_attr_en(attr.v[0].v[1]);
                if (BOOL_ATTRS.includes(attr_name)) {
                  page += ` ${attr_name} `;
                } else {
                  page += ` ${attr_name}= `;
                  if (attr.v[1].id === "str") {
                    page += `'${attr.v[1].v.v[1]}'`;
                  } else if (attr.v[1].id === "int" || attr.v[1].id === "float") {
                    const num = to_maghrib_num$1(attr.v[1].v[0].v[1]);
                    const suffix = attr.v[1].v[1] ? CSS_suffix_en(attr.v[1].v[1].v[1]) || "" : "";
                    page += `${num}${suffix}`;
                  } else if (attr.v[1].id === "bool") {
                    page += `${HTML_attr_en(attr_name)}`;
                  } else {
                    panic$1("not supported: " + to_str$1(attr));
                  }
                }
              } else {
                panic$1("not supported: " + to_str$1(attr));
              }
            });
            page += ">";
            children.forEach((c) => {
              page = this.write_ar_html(c, page);
            });
            page += `</${tag}>`;
        }
        break;
      case "str":
        page += el.v.v[1];
        break;
      default:
        panic$1("unknown html element: " + to_str$1(el));
    }
    return page;
  }
  write_ar_css(attrs, page) {
    attrs.forEach((attr) => {
      const k = maybe_hyphenated(CSS_key_en(attr.v[0].v[1]));
      const v = attr.v[1];
      if (k === "element") {
        page = this.write_ar_css_selector(v, page);
        page += " {";
      } else {
        page += `${k} : `;
        page = this.write_ar_css_attr_value(v, page);
        page += `; `;
      }
    });
    return page;
  }
  write_ar_css_selector(v, page) {
    const translate = (path) => {
      const get_regexp = (k) => RegExp(`(?<![p{L}\\p{N}_])${k}(?![\\p{L}\\p{N}_])`, "ug");
      Object.keys(HTML_tag_en).forEach((k) => {
        path = path.replaceAll(get_regexp(k), HTML_tag_en[k]);
      });
      Object.keys(CSS_pseudo_en).forEach((k) => {
        path = path.replaceAll(get_regexp(k), CSS_pseudo_en[k]);
      });
      return path;
    };
    if (is_list$1(v.v)) {
      page += " ";
      v.v.forEach((selector, i) => {
        let path = selector.v.v[1];
        page += translate(path);
        if (i < v.v.length - 1) {
          page += ",";
        }
      });
    } else {
      const path = v.v.v[1];
      page += translate(path);
    }
    return page;
  }
  write_ar_css_attr_value(v, page) {
    switch (v.id) {
      case "bool":
        panic$1();
        break;
      case "int":
      case "float":
        const num = to_maghrib_num$1(v.v[0].v[1]);
        const suffix = CSS_suffix_en(v.v[1] && v.v[1].v[1]) || "";
        page += num + suffix;
        break;
      case "prefix":
        page += v.v.op.v;
        page = this.write_ar_css_attr_value(v.v.opr, page);
        break;
      case "postfix":
        page = this.write_ar_css_attr_value(v.v.opr, page);
        page += v.v.op.v;
        break;
      case "str":
        page += CSS_str_en(v.v.v[1]);
        break;
      case "ref":
        page += maybe_hyphenated(CSS_value_en(v.v.v[1]));
        break;
      case "tuple":
        v.v.forEach((el) => {
          page = this.write_ar_css_attr_value(el, page + " ");
        });
        break;
      case "call":
        const ref = CSS_fn_en(v.v[0].v.v[1]);
        const args = v.v[1];
        page += ` ${ref}(`;
        args.forEach((arg) => {
          page = this.write_ar_css_attr_value(arg, page);
        });
        page += `)`;
        break;
      case "bin":
        this.jsGen.write_expr(v);
        const code = this.jsGen.get_code();
        page += "${" + code + "}".trim();
        break;
      default:
        panic$1(`not supported: html generations:  ${to_str$1(v)}`);
    }
    return page;
  }
  write_ar_css_fontface(attrs, page) {
    page += `@font-face { `;
    page = this.write_ar_css(attrs, page);
    page += "}";
    return page;
  }
  write_ar_css_keyframes(attrs, children, page) {
    page += ` @keyframes ${attrs[0].v.v[1]} { `;
    children && children.forEach((c) => {
      const ref = c.v[0].v.v[1];
      const v = CSS_value_en(c.v[1]);
      switch (ref) {
        case "عند":
          const percentage = v[0];
          const attrs2 = v[1].v || [];
          page = this.write_ar_css_attr_value(percentage, page);
          page += " {";
          attrs2.forEach((attr) => {
            if (attr.id === "named_tuple") {
              attr.v.forEach((el) => {
                const _k = maybe_hyphenated(CSS_key_en(el[0].v[1]));
                const _v = CSS_value_en(el[1]);
                page += ` ${_k} : `;
                page = this.write_ar_css_attr_value(_v, page);
                page += `; `;
              });
            } else {
              const _k = maybe_hyphenated(CSS_key_en(attr[0].v[1]));
              const _v = CSS_value_en(attr[1]);
              page += ` ${_k} : `;
              page = this.write_ar_css_attr_value(_v, page);
              page += `; `;
            }
          });
          page += "} ";
          break;
        default:
          panic$1("unsupported element: " + to_str$1(ref));
      }
    });
    page += "}";
    return page;
  }
}
function maybe_hyphenated(id) {
  if (HYPHENATED.includes(id)) {
    return id.replaceAll("_", "-");
  } else {
    return id;
  }
}
const MAGHRIB_DIGIT = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
function is_maghrib_num(n) {
  return contains(MAGHRIB_DIGIT, n[0]);
}
function to_maghrib_num(n) {
  if (!is_maghrib_num(n)) {
    let v = "";
    let i = 0;
    while (i < n.length) {
      const c = n[i];
      switch (c) {
        case "٠":
          v += "0";
          break;
        case "١":
          v += "1";
          break;
        case "٢":
          v += "2";
          break;
        case "٣":
          v += "3";
          break;
        case "٤":
          v += "4";
          break;
        case "٥":
          v += "5";
          break;
        case "٦":
          v += "6";
          break;
        case "٧":
          v += "7";
          break;
        case "٨":
          v += "8";
          break;
        case "٩":
          v += "9";
          break;
        case ",":
          v += ".";
          break;
        default:
          panic();
      }
      i += 1;
    }
    return v;
  } else {
    return n;
  }
}
function is_list(x) {
  return x instanceof Array;
}
function contains(list, el) {
  return list.includes(el);
}
function to_str(obj, indent) {
  let objects = [];
  const eliminateCircular = (k, v) => {
    if (v && typeof v === "object") {
      if (objects.includes(v)) {
        return "[CIRCULAR]";
      } else {
        objects.push(v);
      }
    }
    return v;
  };
  {
    return JSON.stringify(obj, eliminateCircular);
  }
}
function panic(v) {
  throw new Error(v);
}
function repeat(str, times) {
  return str.repeat(times);
}
const HELPERS = `
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
function اطبع_سطر(v) { println(v) }
function println(v) {         
    if(v == null ) { console.log("undefined") } else { console.log(v) }
}
function panic(v) { throw new Error(v)}
function clone(obj) { return {...obj} }
function contains(list, el) { return list.includes(el) }
function is_empty(list) { return Array.isArray(list) && list.length === 0 }
function اطبع_تفاصيل(obj, indent) { pprint(obj, indent) }
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
function عرض_اولي(code, preview_id){ preview(code, preview_id) }
function preview(code, preview_id) { window.parent.document.querySelector(preview_id).srcdoc = code }
function هات_الافرع(س) {
    return س.children
}
function اختر(س,دالة) {
    return س.filter(دالة)
}
function هات(ق,فهرس) { return ق[فهرس]}
function عدد_العناصر(ق) { return ق.length}
async function read_url(url) {
    const response = await fetch(url);
    return response.text()
}

//------------------------------------------------------------------------------
`;
const AR_ID = {
  "بدء": "main",
  "اطبع_سطر": "println",
  "تعبير_نمطي": "regex",
  "هذا": "this",
  "مشيّد": "constructor",
  "انهاء": "panic"
};
const SPACES = 4;
class JSGen {
  current;
  indent_level;
  stack;
  astructs;
  ast;
  symtab;
  html_gen;
  main_args;
  opts;
  runtime;
  current_instance;
  init(lang, ast, symtab, html_gen, main_args, opts) {
    this.current = "";
    this.indent_level = 0;
    this.stack = [];
    this.astructs = [];
    this.ast = ast;
    this.symtab = symtab;
    this.html_gen = html_gen;
    this.main_args = main_args;
    this.opts = opts;
    this.current_instance = null;
  }
  init_with_HTML(lang, html_gen, ast, symtab, main_args, opts) {
    this.init(lang, ast, symtab, html_gen, main_args, opts);
    this.html_gen = html_gen;
  }
  init_with_runtime(lang, runtime, ast, symtab, main_args, opts) {
    this.init(lang, ast, symtab, null, main_args, opts);
    this.runtime = runtime;
  }
  run() {
    this.strict_mode();
    let main;
    let i = 0;
    while (i < this.ast.length) {
      const n = this.ast[i];
      if (n) {
        const v = n.v;
        switch (n.id) {
          case "use":
            this.write_use(v);
            break;
          case "modif":
            this.write_modifier(v);
            break;
          case "main":
            main = v;
            break;
          case "const":
            this.write_const(v);
            break;
          case "fn":
            this.write_fn(v);
            break;
          case "struct":
            this.write_struct(v);
            break;
          case "enum":
            this.write_enum(v);
            break;
          case "receiver":
            break;
          default:
            panic("unsupported node: " + this.ast[i].id);
        }
      }
      i += 1;
    }
    this.write_helper_fns();
    if (main) {
      this.write_main(main);
    }
    const code = this.get_code();
    return code;
  }
  to_en_id(id) {
    if (!id.v && !is_list(id.v)) {
      return;
    }
    if (AR_ID[id.v[1]]) {
      id.v[1] = AR_ID[id.v[1]];
    }
  }
  push() {
    this.stack.push(this.current);
    this.current = "";
  }
  pop() {
    this.current = this.stack.pop() + this.current;
  }
  pop_prepend() {
    this.current = this.current + this.stack.pop();
  }
  prepend(code) {
    this.current = code + this.current;
  }
  append(code) {
    this.current += code;
  }
  appendi(code) {
    this.current += this.spaces();
    this.current += code;
  }
  spaces(level) {
    if (!level) {
      level = this.indent_level;
    }
    return repeat(" ", level * SPACES);
  }
  strict_mode() {
    this.append('"use strict";\n\n');
  }
  write_id_pat(id) {
    const v = id.v.v[1];
    this.append(v === "_" ? "default" : v);
  }
  write_char_pat(c) {
    this.append("'" + c.v.v[1] + "'");
  }
  write_str_pat(str) {
    this.append('"' + str.v.v[1] + '"');
  }
  write_tuple_pat(tuple) {
    this.append("(");
    let i = 0;
    while (i < tuple.v.length) {
      this.write_pat(tuple.v[i]);
      if (i < tuple.v.length - 1) {
        this.append(", ");
      }
      i += 1;
    }
    this.append(")");
  }
  write_pat(p) {
    switch (p.id) {
      case "id":
        this.write_id_pat(p);
        break;
      case "bool":
        this.append(p.v.v[1]);
        break;
      case "int":
      case "float":
        this.append(to_maghrib_num(p.v.v[1][0]));
        break;
      case "char":
        this.write_char_pat(p);
        break;
      case "str":
        this.write_str_pat(p);
        break;
      case "tuple":
        this.write_tuple_pat(p);
        break;
      case "_":
        this.append("default");
        break;
      default:
        panic("unsupported pattern " + to_str(p));
    }
  }
  write_modifier(n) {
    if (this.opts.ignore_export) {
      return;
    }
    if (n.v === "+") {
      this.appendi("export ");
    }
  }
  write_use(n) {
    return;
  }
  write_main(_fn) {
    this.push();
    this.appendi("(");
    this.write_fn(_fn, this.main_args);
    this.appendi(")()\n");
    this.pop();
  }
  write_params(params) {
    this.append("(");
    let i = 0;
    while (i < params.length) {
      if (i > 0) {
        this.append(", ");
      }
      this.write_pat(params[i].v._pat);
      i += 1;
    }
    this.append(")");
  }
  write_do_block(block) {
    this.append(`(()=>`);
    this.write_block(block);
    this.append(`)() 
`);
  }
  write_block(block) {
    this.append(" {\n");
    this.push();
    this.indent_level += 1;
    let i = 0;
    const length = block.v.length;
    while (i < length) {
      const stmt = block.v[i];
      this.write_stmt(stmt);
      i += 1;
    }
    this.pop();
    this.indent_level -= 1;
    this.appendi("}\n");
  }
  write_fn(_fn, main_args) {
    this.push();
    if (_fn.t === "fn") {
      this.appendi("static ");
    }
    this.to_en_id(_fn.name);
    if (_fn.is_async) {
      this.append("async ");
    }
    this.append("function " + _fn.name.v[1]);
    if (main_args) {
      this.append("()");
    } else {
      this.write_params(_fn.params);
    }
    this.write_body(_fn.body, _fn.name === "main", main_args);
    this.pop();
  }
  write_method(_fn, instance) {
    this.push();
    this.to_en_id(_fn.name);
    if (_fn.is_async) {
      this.append("async ");
    }
    this.append(_fn.name.v[1]);
    this.write_params(_fn.params);
    this.current_instance = instance;
    this.write_body(_fn.body, false);
    this.current_instance = null;
    this.pop();
  }
  write_fields(fields) {
    const ids = [];
    fields.forEach((field) => {
      const id = field.v[0].v[1];
      ids.push(id);
    });
    ids.forEach((id) => {
      this.appendi(this.spaces() + "" + id + "\n");
    });
    this.write_init(ids);
  }
  write_init(ids) {
    this.append("\n");
    this.appendi("constructor(");
    let i = 0;
    while (i < ids.length) {
      this.append(ids[i]);
      if (i < ids.length - 1) {
        this.append(", ");
      }
      i += 1;
    }
    this.append(") {\n");
    this.indent_level += 1;
    i = 0;
    while (i < ids.length) {
      this.appendi("this." + ids[i] + " = " + ids[i] + "\n");
      i += 1;
    }
    this.appendi("return this\n");
    this.indent_level -= 1;
    this.appendi("}\n");
  }
  // write_typedef(_typedef) {
  //     this.appendi("class " + _typedef.name.v[1] + " {\n")
  //     this.indent_level += 1
  //     if(_typedef.fields) { this.write_fields(_typedef.fields) }
  //     let fns = this.symtab.receivers[_typedef.name.v[1]]
  //     fns && fns.forEach( (data) => {
  //         const fn = data[0]
  //         const instance = data[1]
  //         this.write_method(fn.v, instance) // FIXME: names are confusing , write_fn is handling fn.v, not fn 
  //     })
  //     this.append('child(x) { return this.children[x] }')
  //     this.append('children() { return this.children }')
  //     this.indent_level -= 1
  //     this.appendi("}\n\n")
  // }
  write_struct(_struct) {
    this.appendi("class " + _struct.name.v[1] + " {\n");
    if (_struct.fields) {
      this.write_fields(_struct.fields);
    }
    let fns = this.symtab.receivers[_struct.name.v[1]];
    fns && fns.forEach((data) => {
      const fn = data[0];
      const instance = data[1];
      this.write_method(fn.v, instance);
    });
    this.append("sn__(x) { return this.sn__[x] }");
    this.append("sn__() { return this.sn__}");
    this.appendi("}\n\n");
  }
  write_enum(_enum) {
    panic("enum is not implemented yet.");
  }
  write_const(_const) {
    this.appendi("const ");
    this.write_pat(_const.lhs);
    this.append(" = ");
    this.write_expr(_const.rhs);
    this.append("\n");
  }
  write_var(_var) {
    this.appendi("let ");
    this.write_pat(_var.lhs);
    if (_var.rhs) {
      this.append(" = ");
      this.write_expr(_var.rhs);
    }
    this.append("\n");
  }
  write_ret(n) {
    this.append("return ");
    if (n.v) {
      this.write_expr(n.v);
    }
  }
  write_break(expr) {
    this.append("break");
  }
  write_stmt(stmt) {
    if (stmt.t === "expr") {
      this.appendi("");
      this.write_expr(stmt);
      this.append("\n");
    } else if (stmt.id === "const") {
      this.write_const(stmt.v);
    } else if (stmt.id === "var") {
      this.write_var(stmt.v);
    } else if (stmt.id === "break") {
      this.write_break(stmt);
    } else {
      panic("cannot write stmt: " + to_str(stmt));
    }
  }
  write_body(body, is_main, main_args) {
    this.append(" {\n");
    this.push();
    this.indent_level += 1;
    if (main_args) {
      for (const [k, v] of Object.entries(main_args)) {
        this.append(`const ${k} = '${v}'
`);
      }
    }
    let i = 0;
    const length = body.v.length;
    while (i < length) {
      const stmt = body.v[i];
      this.write_stmt(stmt);
      i += 1;
    }
    this.pop();
    this.indent_level -= 1;
    this.appendi("}\n");
  }
  write_id(id) {
    this.append(id.v[1]);
  }
  write_ref(expr) {
    const _ref = expr.v.v[1];
    if (_ref === this.current_instance) {
      this.append("this");
    } else {
      this.append(_ref);
    }
  }
  write_str(expr) {
    const str = expr.v.v[1];
    const symbol2 = str.indexOf("${") === -1 ? '"' : "`";
    this.append(symbol2 + str + symbol2);
  }
  write_str_id(expr) {
    this.append(symbol + expr.v.v[1] + symbol);
  }
  is_call(expr) {
    return expr.v.id === "bin" && expr.v.v.op.v === "(";
  }
  write_iret(expr) {
    if (!(contains(["when", "while", "if", "for", "return"], expr.v.node) || expr.v.t === "()" || expr.v.t === "void" || expr.v.t === "" || this.is_call(expr) || expr.semicolon)) {
      this.append("return ");
    }
    if (this.is_call(expr)) {
      this.append("const temp_seen_var = ");
      this.write_expr(expr.v);
      this.append("\n");
      this.append("return temp_seen_var");
    } else {
      this.write_expr(expr.v);
    }
  }
  write_list(expr) {
    this.append("[");
    let i = 0;
    const length = expr.v.length;
    while (i < length) {
      this.write_expr(expr.v[i]);
      if (i < expr.v.length - 1) {
        this.append(", ");
      }
      i += 1;
    }
    this.append("]");
  }
  write_structl(expr) {
    const fields = expr.v;
    this.append("{");
    let i = 0;
    while (i < fields.length) {
      const field = fields[i];
      const key = field.k;
      if (key.v.v[1]) {
        this.write_id(key.v);
      } else {
        this.write_str_id(key.v);
      }
      const value = field.v;
      this.append(": ");
      this.write_expr(value);
      if (i < fields.length - 1) {
        this.append(", ");
      }
      i += 1;
    }
    this.append("}");
  }
  write_args(expr) {
    this.append("(");
    let i = 0;
    while (i < expr.v.length) {
      let _expr = expr.v[i];
      if (_expr.v.op && _expr.v.op.v === ":") {
        _expr = _expr.v.ropr;
      }
      this.write_expr(_expr);
      if (i < expr.v.length - 1) {
        this.append(", ");
      }
      i += 1;
    }
    this.append(")");
  }
  write_named_arg(narg) {
    narg.v[0].v[1];
    const v = narg.v[1];
    this.write_expr(v);
  }
  write_tuple(expr) {
    this.append("[");
    let i = 0;
    while (i < expr.v.length) {
      let arg = expr.v[i];
      if (arg.id === "narg") {
        arg = expr.v[i].v[1];
      }
      this.write_expr(arg);
      if (i < expr.v.length - 1) {
        this.append(", ");
      }
      i += 1;
    }
    this.append("]");
  }
  write_named_tuple(expr) {
    this.append("{");
    expr.v.forEach((pair, i) => {
      const k = pair[0].v[1];
      const v = pair[1];
      this.append(k);
      this.append(": ");
      this.write_expr(v);
      if (i < expr.v.length) {
        this.append(",");
      }
    });
    this.append("}");
  }
  write_when(expr) {
    this.appendi("switch(");
    this.write_expr(expr.v.expr);
    this.append(") {\n");
    this.indent_level += 1;
    let i = 0;
    while (i < expr.v.arms.length) {
      const arm = expr.v.arms[i];
      const pats = arm.v.pats;
      const _expr = arm.v.expr;
      let j = 0;
      while (j < pats.length) {
        if (pats[j].id !== "_") {
          this.appendi("case ");
        }
        this.write_pat(pats[j]);
        this.append(" :\n");
        j += 1;
      }
      this.indent_level += 1;
      this.appendi("");
      this.write_expr(_expr);
      this.append("\n");
      this.appendi("break\n");
      this.indent_level -= 1;
      i += 1;
    }
    this.indent_level -= 1;
    this.appendi("}\n");
  }
  write_prefix_uni(expr) {
    const op = expr.v.op.v;
    switch (op) {
      case ".":
        {
          if (expr.v.opr.v.v[1] === "none") {
            this.append("null");
            return;
          } else {
            panic("enum variants are not supported, found : (." + expr.v.opr.v.v[1] + ")");
          }
        }
        break;
      case "not":
        this.append("!");
        break;
      case "!":
      case "-":
        this.append(op);
        break;
      default:
        panic("unsupported op: " + op);
        break;
    }
    this.write_expr(expr.v.opr);
  }
  write_pipe(stack) {
    while (stack.length > 0) {
      let expr = stack.pop();
      switch (expr.id) {
        case "ref":
          this.write_expr(expr);
          if (stack.length > 0) {
            this.append("(");
            this.write_pipe(stack);
            this.append(")");
          }
          break;
        case "call":
          const lhs = expr.v[0];
          const rhs = expr.v[1];
          this.write_expr(lhs);
          this.append("(");
          this.write_pipe(stack);
          if (this.current.slice(-1) !== "(" && rhs.length > 0) {
            this.append(", ");
          }
          rhs.forEach((el, i) => {
            this.write_expr(el);
            if (i < rhs.length - 1) {
              this.append(", ");
            }
          });
          this.append(")");
          break;
        case "int":
        case "float":
        case "str":
        case "[":
        case "tuple":
        case "named_tuple":
          this.write_expr(expr);
          break;
        default:
          throw new Error("syntax error |> :" + to_str(expr));
      }
    }
  }
  write_runtime_fn() {
  }
  write_call(expr) {
    const runtime_impl = this.runtime && this.runtime.get_fn(expr);
    if (runtime_impl) {
      if (runtime_impl._import) {
        this.prepend(runtime_impl._import);
      }
      this.append(runtime_impl.code);
      return;
    }
    this.to_en_id(expr.v[0].v);
    if (expr.v[0].v.v[1] === "html") {
      const page = this.html_gen.en.write_html(expr, "");
      this.append(` (() => \`${page}\`)() `);
      return;
    } else if (expr.v[0].v.v[1] === "صفحة_الشبكة") {
      const page = this.html_gen.ar.write_ar_html(expr, "");
      this.append(`(() => \`${page}\`)()`);
      return;
    } else if (this.symtab.structs.includes(expr.v[0].v.v[1])) {
      this.append("new ");
    }
    this.write_expr(expr.v[0]);
    this.append("(");
    const args = expr.v[1];
    if (args) {
      args.forEach((arg, i) => {
        this.write_expr(arg);
        if (i < args.length - 1) {
          this.append(", ");
        }
      });
    }
    if (expr.v[2]) {
      if (args) {
        this.append(", ");
      }
      this.write_children(expr.v[2]);
    }
    this.append(")");
  }
  write_children(block) {
    if (!block || block.length === 0) {
      return;
    }
    this.append("[");
    block.forEach((expr) => {
      this.write_expr(expr);
      this.append(",");
    });
    this.append("]");
  }
  write_bin(expr) {
    const op = expr.v.op.v;
    switch (op) {
      case "[":
        this.write_expr(expr.v.lopr);
        this.append("[");
        this.write_expr(expr.v.ropr);
        this.append("]");
        break;
      case "=":
        this.write_expr(expr.v.lopr);
        this.append("=");
        this.write_expr(expr.v.ropr);
        break;
      case ":":
        this.appendi("let ");
        this.write_expr(expr.v.lopr);
        this.append("\n");
        break;
      case "++":
        this.write_expr(expr.v.lopr);
        this.append("+");
        this.write_expr(expr.v.ropr);
        break;
      case "|>":
        throw new Error("|> not implemented");
      case "||>":
        throw new Error(" ||> : WIP , " + to_str(expr));
      case ":>":
        throw new Error(" :> : WIP , " + to_str(expr));
      case "==":
      case "!=":
      case "<":
      case "<=":
      case ">":
      case ">=":
      case "|":
      case "||":
      case "&":
      case "&&":
      case "+":
      case "-":
      case "/":
      case "*":
      case "+=":
      case "-=":
      case "*=":
      case "\\=":
      case ".":
        this.write_expr(expr.v.lopr);
        this.append(op);
        if (op === "==" || op === "!=") {
          this.append("=");
        }
        this.write_expr(expr.v.ropr);
        break;
      default:
        panic("cannot write binary operation: " + to_str(expr));
        break;
    }
  }
  write_afn(expr) {
    this.push();
    this.write_params(expr.v.params);
    this.append("=>");
    this.write_body(expr.v.body);
    this.pop();
  }
  write_expr(expr) {
    if (expr.grouped) {
      this.append("(");
    }
    switch (expr.id) {
      case "void":
        this.append("null");
        break;
      case ";":
        break;
      case "ref":
        this.write_ref(expr);
        break;
      case "bool":
        this.append(expr.v.v[1]);
        break;
      case "int":
      case "float":
        this.append(to_maghrib_num(expr.v[0].v[1]));
        break;
      case "char":
        this.append("'" + expr.v.v[1] + "'");
        break;
      case "str":
        this.write_str(expr);
        break;
      case "return":
        this.write_ret(expr);
        break;
      case "iret":
        this.write_iret(expr);
        break;
      case "[":
        this.write_list(expr);
        break;
      case "{":
        this.write_structl(expr);
        break;
      case "args":
        this.write_args(expr);
        break;
      case "named_arg":
        this.write_named_arg(expr);
        break;
      case "tuple":
        this.write_tuple(expr);
        break;
      case "named_tuple":
        this.write_named_tuple(expr);
        break;
      case "when":
        this.write_when(expr);
        break;
      case "do_block":
        this.write_do_block(expr);
        break;
      case "block":
        this.write_block(expr);
        break;
      case "prefix":
        this.write_prefix_uni(expr);
        break;
      case "call":
        this.write_call(expr);
        break;
      case "bin":
        this.write_bin(expr);
        break;
      case "afn":
        this.write_afn(expr);
        break;
      default:
        panic("cannot write expr: " + to_str(expr));
    }
    if (expr.grouped) {
      this.append(")");
    }
  }
  write_helper_fns() {
    this.append(HELPERS);
  }
  get_code() {
    return this.current;
  }
}
class HtmlCssJSGen {
  run(lang, ast, symtab, main_args, opts) {
    const en_html = new HtmlWriter(new JSGen());
    const ar_html = new ArHtmlWriter(new JSGen());
    const html_gen = {
      en: en_html,
      ar: ar_html
    };
    const js_gen = new JSGen();
    js_gen.init(
      lang,
      ast,
      symtab,
      html_gen,
      main_args,
      opts
    );
    return js_gen.run();
  }
}
export {
  HtmlCssJSGen
};
