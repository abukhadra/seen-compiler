class V {
  msg;
  start_loc;
  end_loc;
  constructor(t, e, s) {
    this.msg = t, this.start_loc = e, this.end_loc = s;
  }
}
function Z(r) {
  return p(b, r[0]);
}
function $(r) {
  if (Z(r))
    return r;
  {
    let t = "", e = 0;
    for (; e < r.length; ) {
      switch (r[e]) {
        case "٠":
          t += "0";
          break;
        case "١":
          t += "1";
          break;
        case "٢":
          t += "2";
          break;
        case "٣":
          t += "3";
          break;
        case "٤":
          t += "4";
          break;
        case "٥":
          t += "5";
          break;
        case "٦":
          t += "6";
          break;
        case "٧":
          t += "7";
          break;
        case "٨":
          t += "8";
          break;
        case "٩":
          t += "9";
          break;
        case ",":
          t += ".";
          break;
        default:
          o();
      }
      e += 1;
    }
    return t;
  }
}
function q(r) {
  return r == null;
}
function g(r) {
  return r instanceof Array;
}
function p(r, t) {
  return r.includes(t);
}
function tt(r, t, e) {
  r[t] = e;
}
function N(r) {
  return Array.isArray(r) && r.length === 0;
}
function _(r, t) {
  let e = [];
  return JSON.stringify(r, (i, n) => {
    if (n && typeof n == "object") {
      if (e.includes(n))
        return "[CIRCULAR]";
      e.push(n);
    }
    return n;
  });
}
function G(r) {
  return new RegExp(r, "u");
}
function O(r, t) {
  r == null ? console.log("undefined") : console.log(JSON.stringify(r));
}
function o(r) {
  throw new Error(r);
}
function m(r) {
  return { ...r };
}
function et(r, t) {
  return r.repeat(t);
}
function st(r) {
  return r.toLowerCase();
}
const y = ["٠", "١", "٢", "٣", "٤", "٥", "٦", "٧", "٨", "٩"], b = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"], it = "ـ", S = G("\\p{L}"), rt = G("\\p{N}"), R = {
  "???": "as",
  صحيح: "true",
  غير_صحيح: "false",
  عندما: "when",
  حيث: "where",
  احضر: "use",
  عرف: "let",
  ثابت: "const",
  متغير: "var",
  دل: "fn",
  نوع: "type",
  سمة: "trait",
  "???": "impl",
  رد: "return",
  نفذ: "do"
}, nt = {
  as: "???",
  true: "صحيح",
  false: "غير_صحيح",
  when: "عندما",
  where: "حيث",
  use: "احضر",
  let: "عرف",
  const: "ثابت",
  var: "متغير",
  fn: "دل",
  type: "نوع",
  trait: "سمة",
  impl: "???",
  return: "رد",
  do: "نفذ"
};
class L {
  line;
  column;
  constructor(t, e) {
    return this.line = t, this.column = e, this;
  }
}
class at {
  v;
  loc;
  constructor(t, e) {
    return this.v = t, this.loc = e, this;
  }
}
class ht {
  lang;
  code;
  start_loc;
  end_loc;
  tokens;
  errs;
  current_index;
  current;
  lookbehind;
  ignore_cmts_ws;
  constructor(t, e, s, i, n, a, u, c, l, d) {
    return this.lang = t, this.code = e, this.start_loc = s, this.end_loc = i, this.tokens = n, this.errs = a, this.current_index = u, this.current = c, this.lookbehind = l, this.ignore_cmts_ws = d, this;
  }
  init(t, e, s) {
    this.lang = t, this.code = e, this.start_loc = new L(1, 1), this.end_loc = new L(1, 1), this.tokens = [], this.errs = [], this.current_index = -1, this.current = null, this.lookbehind = null, this.ignore_cmts_ws = s;
  }
  run() {
    this.lang === "ar" ? this.ar() : this.lang === "en" && this.en(), this.start_loc = m(this.end_loc), this.add_token("$eof");
  }
  add_token(t) {
    const e = new at(t, m(this.start_loc));
    this.tokens.push(e);
  }
  next() {
    this.lookbehind = this.current, this.current_index += 1;
    const t = this.code[this.current_index];
    return this.current = t, t === `
` ? (this.end_loc.line += 1, this.end_loc.column = 1) : this.end_loc.column += 1, this.current;
  }
  lookahead() {
    return this.code[this.current_index + 1];
  }
  skip(t) {
    for (; t > 0; )
      this.next(), t -= 1;
  }
  insert_err(t) {
    const e = new V(t, m(this.start_loc), m(this.end_loc));
    this.errs.push(e);
  }
  last_token() {
    return this.tokens[this.tokens.length - 1];
  }
  skip_invalid_num_or_id() {
    for (; this.expect_letter() || this.expect_num() || this.expect_underscore(); )
      this.skip(1);
  }
  expect_tatweel() {
    return this.lookahead() === it;
  }
  expect_nl_behind() {
    return this.lookbehind === `
`;
  }
  expect_none_behind() {
    return q(this.lookbehind);
  }
  expect_none_ahead() {
    return q(this.lookahead());
  }
  expect_ws_behind() {
    return this.lookbehind === `
` || this.lookbehind === " " || this.lookbehind === "\r" || this.lookbehind === "	";
  }
  expect_space_ahead() {
    return this.lookahead() === " " || this.lookahead() === "	";
  }
  expect_ws_ahead() {
    return this.lookahead() === `
` || this.lookahead() === " " || this.lookahead() === "\r" || this.lookahead() === "	";
  }
  expect_nl_ahead() {
    return this.lookahead() === `
`;
  }
  expect_separator_behind() {
    return this.lookbehind === "," || this.lookbehind === ";" || this.lookbehind === ";;" || this.lookbehind === ":" || this.lookbehind === "(" || this.lookbehind === "[" || this.lookbehind === "{" || this.lookbehind === "<";
  }
  expect_separator_ahead() {
    return this.lookahead() === "," || this.lookahead() === ";" || this.lookahead() === ";;" || this.lookahead() === ":" || this.lookahead() === ")" || this.lookahead() === "]" || this.lookahead() === "}" || this.lookahead() === ">";
  }
  expect_open_bracket() {
    return this.lookahead() === "[";
  }
  expect_open_paren() {
    return this.lookahead() === "(";
  }
  expect_letter() {
    if (this.lookahead())
      return this.lookahead().match(S);
  }
  expect_num() {
    return this.lookahead().match(rt);
  }
  expect_underscore() {
    return this.lookahead() === "_";
  }
  expect_eof() {
    return q(this.lookahead());
  }
  expect_eol() {
    return this.lookahead() === `
` || this.lookahead() === "\r" || this.expect_eof();
  }
  multi_comment() {
    if (this.lookahead() === "-") {
      let t = "";
      const e = [];
      for (; !this.expect_eof(); )
        if (this.current === "{" && this.lookahead() === "-")
          t += this.next() + this.next(), e.push(m(this.end_loc));
        else if (this.current === "-" && this.lookahead() === "}")
          if (e.length > 1)
            t += this.next() + this.next(), e.pop();
          else {
            t += this.next(), e.pop();
            break;
          }
        else
          t += this.next();
      const s = e.pop();
      return s && (this.start_loc = m(s), this.insert_err("unclosed comment")), this.ignore_cmts_ws || this.add_token(["--", t]), !0;
    }
  }
  ar_escape_char() {
    if (this.current === "٪") {
      let e = this.next();
      switch (e) {
        case "(":
          switch (e = this.next(), e) {
            case "س":
              e = `
`;
              break;
            case "ر":
              e = "\r";
              break;
            case "ج":
              e = "	";
              break;
            case "‹":
              e = "‹";
              break;
            case "›":
              e = "›";
              break;
            case "«":
              e = "«";
              break;
            case "»":
              e = "»";
              break;
          }
          this.next() !== ")" && this.insert_err("invalid_escape_character: " + e);
          break;
        case "٪":
          e = "٪";
          break;
        case "{":
          e = "${";
          break;
        default:
          this.insert_err("invalid escape character: " + e);
      }
      return e;
    } else
      return this.current;
  }
  en_escape_char() {
    if (this.current === "%") {
      let e = this.next();
      switch (e) {
        case "n":
          e = `
`;
          break;
        case "r":
          e = "\r";
          break;
        case "t":
          e = "	";
          break;
        case "'":
          e = "'";
          break;
        case '"':
          e = '"';
          break;
        case "%":
          e = "%";
          break;
        case "{":
          e = "${";
          break;
        default:
          this.insert_err("invalid escape character: " + e);
          break;
      }
      return e;
    } else
      return this.current;
  }
  enclosed_val(t) {
    let e = "";
    for (; !this.expect_eof() && (this.next(), t !== this.current); ) {
      if (this.expect_eol()) {
        this.insert_err("unclosed literal, expecting: " + t);
        break;
      }
      this.lang === "ar" ? e += this.ar_escape_char() : e += this.en_escape_char();
    }
    return e;
  }
  ar_str() {
    this.lookahead() === "«" ? this.ar_multi_str() : this.add_token(["str", this.enclosed_val("»")]);
  }
  en_str() {
    this.lookahead() === '"' ? this.en_multi_str() : this.add_token(["str", this.enclosed_val("'")]);
  }
  multi_str(t) {
    let e = "", s = "";
    for (; !this.expect_eof(); ) {
      if (this.next(), this.expect_eof()) {
        this.insert_err("unclosed multiline String literal, expecting " + t);
        break;
      }
      if (t === this.current && t === this.lookahead())
        if (this.skip(1), this.lookahead() === t) {
          this.skip(1), this.add_token(["str", s]);
          break;
        } else
          e += t + t;
      else
        this.lang === "ar" ? s += this.ar_escape_char() : s += this.en_escape_char(), s += e;
    }
  }
  ar_multi_str(t) {
    this.skip(1), this.lookahead() === "«" ? (this.skip(1), this.multi_str("»")) : this.add_token(["str", ""]);
  }
  en_multi_str(t) {
    this.skip(1), this.lookahead() === '"' ? (this.skip(1), this.multi_str('"')) : this.add_token(["str", ""]);
  }
  equal() {
    const t = [];
    for (; this.lookahead() === "="; )
      t.push(this.next());
    if (t.length === 0)
      this.add_token("=");
    else if (t.length === 1)
      this.add_token("==");
    else if (t.length > 1)
      this.ignore_cmts_ws || this.add_token(["===", t.length + 1]);
    else
      return !1;
    return !0;
  }
  thick_arrow() {
    if (this.lookahead() === ">")
      return this.next(), this.add_token("=>"), !0;
  }
  add_asgmt() {
    if (this.lookahead() === "=")
      return this.next(), this.add_token("+="), !0;
  }
  add() {
    return this.add_token("+"), !0;
  }
  sub_asgmt() {
    if (this.lookahead() === "=")
      return this.next(), this.add_token("-="), !0;
  }
  thin_arrow() {
    if (this.lookahead() === ">")
      return this.next(), this.add_token("->"), !0;
  }
  dash() {
    const t = [];
    for (; this.lookahead() === "-"; )
      t.push(this.next());
    if (t.length === 0)
      this.add_token("-");
    else if (t.length > 1)
      this.ignore_cmts_ws || this.add_token(["---", t.length + 1]);
    else
      return !1;
    return !0;
  }
  tilde() {
    if (this.lookahead() === "~")
      return this.next(), this.add_token("~"), !0;
  }
  mul_asgmt() {
    if (this.lookahead() === "=")
      return this.next(), this.add_token("*="), !0;
  }
  mul() {
    return this.add_token("*"), !0;
  }
  asterisk() {
    let t = "*";
    for (; this.lookahead() === "*"; )
      t += this.next();
    if (t.length === 1)
      this.mul();
    else
      return !1;
    return !0;
  }
  ar_div_asgmt() {
    if (this.lookahead() === "=")
      return this.next(), this.add_token("\\="), !0;
  }
  div_asgmt() {
    if (this.lookahead() === "=")
      return this.next(), this.add_token("/="), !0;
  }
  comment() {
    if (this.lookahead() === "-") {
      let t = "";
      for (; !this.expect_eof() && !this.expect_eol(); )
        t += this.next();
      return this.ignore_cmts_ws || this.add_token(["--", t]), !0;
    }
  }
  ar_div() {
    return this.add_token("\\"), !0;
  }
  div() {
    return this.add_token("/"), !0;
  }
  ne() {
    if (this.lookahead() === "=")
      return this.next(), this.add_token("!="), !0;
  }
  exclamation() {
    return this.add_token("!"), !0;
  }
  ge() {
    if (this.lookahead() === "=")
      return this.next(), this.add_token(">="), !0;
  }
  gt() {
    return this.add_token(">"), !0;
  }
  le() {
    if (this.lookahead() === "=")
      return this.next(), this.add_token("<="), !0;
  }
  lt() {
    return this.add_token("<"), !0;
  }
  and() {
    return this.lookahead() === "&" ? (this.skip(1), this.add_token("&&"), !0) : (this.add_token("&"), !0);
  }
  or_listpipe() {
    if (this.lookahead() === "|")
      return this.skip(1), this.lookahead() === ">" ? (this.skip(1), this.add_token("||>"), !0) : (this.add_token("||"), !0);
  }
  pipe() {
    if (this.lookahead() === ">")
      return this.skip(1), this.add_token("|>"), !0;
  }
  bar() {
    return this.add_token("|"), !0;
  }
  mashriq_float() {
    p(y, this.lookahead()) ? (v = this.current + this.mashriq_fract(), this.add_token(["float", v])) : this.insert_err("ill-formed floating point number");
  }
  maghrib_float() {
    if (p(b, this.lookahead()))
      return v = this.current + this.maghrib_fract(), this.add_token(["float", v]), !0;
  }
  ddot() {
    if (this.lookahead() === ".")
      return this.skip(1), this.add_token(".."), !0;
  }
  dot() {
    return this.add_token("."), !0;
  }
  deconstruct() {
    if (this.lookahead() === ">")
      return this.skip(1), this.add_token(":>"), !0;
  }
  decl() {
    if (this.lookahead() === "=")
      return this.skip(1), this.add_token(":="), !0;
  }
  dcolon() {
    if (this.lookahead() === ":")
      return this.skip(1), this.lookahead() === "=" ? (this.skip(1), this.add_token("::=")) : this.add_token("::"), !0;
  }
  colon() {
    return this.add_token(":"), !0;
  }
  ar_semicolon() {
    return this.add_token(";"), !0;
  }
  en_semicolon() {
    return this.add_token(";"), !0;
  }
  mashriq_num() {
    let t = this.current;
    for (; ; ) {
      if (p(y, this.lookahead()))
        t += this.next();
      else if (p(b, this.lookahead()))
        this.insert_err("you can either use Mashriq digits (٠ - ٩) or Maghrib digits (0 - 9) but not a mix: " + this.current), this.skip_invalid_num_or_id();
      else if (this.lookahead() === ",")
        t += this.next(), t += this.maghrib_fract();
      else
        break;
      if (this.expect_eof())
        break;
    }
    this.is_float ? this.add_token(["float", t]) : this.add_token(["int", t]);
  }
  maghrib_num() {
    let t = this.current, e = !1;
    for (; ; ) {
      if (p(b, this.lookahead()))
        t += this.next();
      else if (p(y, this.lookahead()))
        this.insert_err("you can either use Eastern Arabic digits (٠ - ٩) or Western (0 - 9) but not a mix: " + this.current), this.skip_invalid_num_or_id();
      else if (this.lookahead() === ".")
        t += this.next(), e = !0, t += this.maghrib_fract();
      else
        break;
      if (this.expect_eof())
        break;
    }
    e ? this.add_token(["float", t]) : this.add_token(["int", t]);
  }
  ar_id() {
    let t = this.current;
    for (; !this.expect_eof() && (this.expect_letter() || this.expect_num() || this.expect_underscore()); )
      this.expect_tatweel() ? p(["ف", "ل", "اه", "ك"], t) ? t += this.next() : this.skip(1) : t += this.next();
    R[t] ? this.add_token(["key", R[t]]) : this.add_token(["id", t]);
  }
  en_id() {
    let t = this.current;
    for (; !this.expect_eof() && (this.expect_letter() || this.expect_num() || this.expect_underscore()); )
      t += this.next();
    nt[t] ? this.add_token(["key", t]) : this.add_token(["id", t]);
  }
  mashriq_fract() {
    let t = "";
    for (; !this.expect_eof() && p(y, this.lookahead()); )
      t += this.next();
    return t;
  }
  maghrib_float() {
    if (p(b, this.lookahead())) {
      const t = this.current + this.maghrib_fract();
      return this.add_token(["float", t]), !0;
    }
  }
  maghrib_fract() {
    let t = "";
    for (; !this.expect_eof() && p(b, this.lookahead()); )
      t += this.next();
    return t;
  }
  new_line() {
    let t = 1;
    for (; ; )
      if (this.lookahead() === `
`)
        this.skip(1), t += 1;
      else {
        this.last_token() && this.last_token().v !== `
` && this.add_token([`
`, t]);
        break;
      }
  }
  ar() {
    for (; !this.expect_eof(); ) {
      this.start_loc = m(this.end_loc);
      const t = this.next();
      switch (t) {
        case "؟":
          this.add_token("?");
          break;
        case "٪":
          this.add_token("%");
          break;
        case ",":
          this.mashriq_float();
          break;
        case ".":
          this.maghrib_float() || this.ddot() || this.dot();
          break;
        case "،":
          this.add_token(",");
          break;
        case "×":
          this.mul_asgmt() || this.mul();
          break;
        case "*":
          this.mul_asgmt() || this.asterisk();
          break;
        case "÷":
        case "/":
          this.div_asgmt() || this.div();
          break;
        case "\\":
          this.ar_div_asgmt() || this.ar_div();
          break;
        case "؛":
          this.ar_semicolon();
          break;
        case "«":
          this.ar_str();
          break;
        default:
          p(y, t) ? this.mashriq_num() : p(b, t) ? this.maghrib_num() : t === "_" || t.match(S) ? this.ar_id() : this.common(t);
      }
    }
  }
  en() {
    for (; !this.expect_eof(); ) {
      this.start_loc = m(this.end_loc);
      const t = this.next();
      switch (t) {
        case "?":
          this.add_token("?");
          break;
        case "%":
          this.add_token("%");
          break;
        case ".":
          this.maghrib_float() || this.ddot() || this.dot();
          break;
        case ",":
          this.add_token(",");
          break;
        case "*":
          this.mul_asgmt() || this.asterisk();
          break;
        case "/":
          this.div_asgmt() || this.div();
          break;
        case ";":
          this.en_semicolon();
          break;
        case "'":
          this.en_str();
          break;
        default:
          p(b, t) ? this.maghrib_num() : p(y, t) ? (this.insert_err("only English Numerals are allowed in English source files: " + this.current), this.skip_invalid_num_or_id()) : t === "_" || t.match(S) ? this.en_id() : this.common(t);
      }
    }
  }
  common(t) {
    switch (t) {
      case `
`:
        this.new_line();
        break;
      case "\r":
      case "	":
      case " ":
        this.ignore_cmts_ws || this.add_token(t);
        break;
      case "&":
        this.and();
        break;
      case "|":
        this.pipe() || this.or_listpipe() || this.bar();
        break;
      case "+":
        this.add_asgmt() || this.add();
        break;
      case "-":
        this.comment() || this.sub_asgmt() || this.thin_arrow() || this.dash();
        break;
      case "~":
        this.tilde();
        break;
      case "^":
        this.add_token("^");
        break;
      case "=":
        this.thick_arrow() || this.equal();
        break;
      case "!":
        this.ne() || this.exclamation();
        break;
      case ">":
        this.ge() || this.gt();
        break;
      case "<":
        this.le() || this.lt();
        break;
      case ":":
        this.deconstruct() || this.decl() || this.dcolon() || this.colon();
        break;
      case "`":
        this.transl();
        break;
      case "$":
        this.add_token("$");
        break;
      case "[":
        this.add_token("[");
        break;
      case "]":
        this.add_token("]");
        break;
      case "(":
        this.add_token("(");
        break;
      case ")":
        this.add_token(")");
        break;
      case "{":
        this.multi_comment() || this.add_token("{");
        break;
      case "}":
        this.add_token("}");
        break;
      default:
        this.insert_err("unrecognized character: " + this.current);
    }
  }
}
const ot = [], U = [];
class h {
  id;
  t;
  v;
  constructor(t, e, s) {
    this.id = t, this.t = e, this.v = s;
  }
}
class P {
  t;
  o;
  constructor(t, e) {
    this.t = t, this.o = e;
  }
}
class ct {
  name;
  fields;
  children;
  constructor(t, e, s) {
    this.name = t, this.fields = e, this.children = s;
  }
}
class _t {
  fields;
  o;
  constructor(t, e) {
    this.fields = t, this.o = e;
  }
}
class lt {
  t;
  ts;
  o;
  constructor(t, e, s) {
    this.t = t, this.ts = e, this.o = s;
  }
}
class ut {
  name;
  variant;
  constructor(t, e) {
    this.name = t, this.variant = e;
  }
}
class z {
  k;
  v;
  constructor(t, e) {
    this.k = t, this.v = e;
  }
}
class f {
  opr;
  op;
  constructor(t, e) {
    this.opr = t, this.op = e;
  }
}
class A {
  lopr;
  op;
  ropr;
  constructor(t, e, s) {
    this.lopr = t, this.op = e, this.ropr = s;
  }
}
class I {
  name;
  params;
  ret_types;
  body;
  constructor(t, e, s, i) {
    this.name = t, this.params = e, this.ret_types = s, this.body = i;
  }
}
class D {
  _pat;
  t;
  constructor(t, e) {
    this._pat = t, this.t = e;
  }
}
class pt {
  k;
  v;
  constructor(t, e) {
    this.k = t, this.v = e;
  }
}
class J {
  lhs;
  t;
  rhs;
  constructor(t, e, s) {
    this.lhs = t, this.t = e, this.rhs = s;
  }
}
class dt {
  expr;
  arms;
  constructor(t, e) {
    this.expr = t, this.arms = e;
  }
}
class ft {
  pats;
  expr;
  constructor(t, e) {
    this.pats = t, this.expr = e;
  }
}
const mt = ["+", "-", "*", "/", "[", "~=", "::", ":=", "=", "+=", "-=", "*=", "/=", "|=", "&=", "==", "!=", ">", ">=", "<", "<=", "|", "||", "|>", "||>", ":>", "&", "&&", ".", ".."], bt = [".", "!", "not", "-", "+"], kt = ["?", "!", "%"], wt = ["=", ":", "::", ":=", "~=", "+=", "-=", "*=", "/=", "÷=", "&=", "&&=", "|=", "||="];
class vt {
  tokens;
  current_index;
  skipped_new_line;
  current;
  ast;
  tabs;
  attrs;
  errs;
  init(t) {
    this.tokens = t, this.current_index = -1, this.skipped_new_line = !1, this.current = null, this.ast = [], this.attrs = [], this.errs = [];
  }
  run() {
    for (this.next(); !this.is_eof(); ) {
      let t = !1;
      this.maybe_attrs() ? (this.maybe_modifier(), t = this.maybe_typedef() || this.maybe_consts_or_fns()) : this.is_modifier() ? (this.maybe_modifier(), t = this.maybe_consts_or_fns() || this.maybe_typedef()) : t = this.maybe_use() || this.maybe_consts_or_fns() || this.maybe_typedef(), t || o("invalid syntax: " + _(this.current));
    }
  }
  next(t) {
    this.current_index += 1;
    const e = this.tokens[this.current_index];
    return this.current = e, e.v[0] === `
` ? (this.skipped_new_line = !0, this.next(!0)) : t || (this.skipped_new_line = !1), this.current;
  }
  backtrack() {
    this.current_index -= 1;
    const t = this.tokens[this.current_index];
    this.current = t, t.v[0] === `
` && this.backtrack();
  }
  skip() {
    return this.next();
  }
  lookahead() {
    return this.lookahead_n(1);
  }
  lookahead_n(t) {
    let e = this.current_index;
    for (; t > 0; ) {
      for (e += 1; ; ) {
        const s = this.tokens[e];
        let i;
        if (s && (i = s.v[0] === `
`), i)
          e += 1;
        else
          break;
      }
      t -= 1;
    }
    return this.tokens[e];
  }
  lookahead_ws() {
    const t = this.current_index + 1;
    return this.tokens[t];
  }
  is_eof() {
    return this.current.v === "$eof";
  }
  is_newline() {
    return this.skipped_new_line;
  }
  is_asterisk() {
    return this.current.v === "*";
  }
  is_asgmt() {
    return this.current.v === "=";
  }
  is_hash() {
    return this.current.v === "--";
  }
  is_percent() {
    return this.current.v === "%";
  }
  is_dpercent() {
    return this.current.v === "%%";
  }
  is_behind_none() {
    return q(this.lookbehind);
  }
  is_behind_nl() {
    return this.lookbehind === `
`;
  }
  is_dot() {
    return this.current.v === ".";
  }
  is_colon() {
    return this.current.v === ":";
  }
  is_dcolon() {
    return this.current.v === "::";
  }
  is_caret() {
    return this.current.v === "^";
  }
  is_semicolon() {
    return this.current.v === ";";
  }
  is_comma() {
    return this.current.v === ",";
  }
  is_backtick() {
    return this.current.v === "`";
  }
  is_tbacktick() {
    return this.current.v[0] === "```";
  }
  is_underscore() {
    return this.is_id() && this.current.v[1] === "_";
  }
  is_plus() {
    return this.current.v === "+";
  }
  is_minus() {
    return this.current.v === "-";
  }
  is_exclamation() {
    return this.current.v === "!";
  }
  is_question() {
    return this.current.v === "?";
  }
  is_bar() {
    return this.current.v === "|";
  }
  is_thin_arrow() {
    return this.current.v === "->";
  }
  is_thick_arrow() {
    return this.current.v === "=>";
  }
  is_tilde() {
    return this.current.v === "~";
  }
  is_or() {
    return this.is_keyword("or");
  }
  is_and() {
    return this.is_keyword("and");
  }
  is_not() {
    return this.is_keyword("not");
  }
  is_use() {
    return this.is_keyword("use");
  }
  is_let() {
    return this.is_keyword("let");
  }
  is_if_let() {
    return this.is_keyword("if_let");
  }
  is_const() {
    return this.is_keyword("const");
  }
  is_var() {
    return this.is_keyword("var");
  }
  is_then() {
    return this.is_keyword("then");
  }
  is_do() {
    return this.is_keyword("do");
  }
  is_end() {
    return this.is_keyword("end");
  }
  is_fn() {
    return this.is_keyword("fn");
  }
  is_alias() {
    return this.is_keyword("alias");
  }
  is_typedef() {
    return this.is_keyword("type");
  }
  is_open_paren() {
    return this.current.v === "(";
  }
  is_close_paren() {
    return this.current.v === ")";
  }
  is_open_curly() {
    return this.current.v === "{";
  }
  is_close_curly() {
    return this.current.v === "}";
  }
  is_open_bracket() {
    return this.current.v === "[";
  }
  is_close_bracket() {
    return this.current.v === "]";
  }
  is_open_angle() {
    return this.current.v === "<";
  }
  is_close_angle() {
    return this.current.v === ">";
  }
  is_double_close_angle() {
    return this.current.v === ">>";
  }
  is_if() {
    return this.is_keyword("if");
  }
  is_else() {
    return this.is_keyword("else");
  }
  is_ret() {
    return this.is_keyword("return");
  }
  is_break() {
    return this.is_keyword("break");
  }
  is_when() {
    return this.is_keyword("when");
  }
  is_for() {
    return this.is_keyword("for");
  }
  is_in() {
    return this.is_keyword("in");
  }
  is_while() {
    return this.is_keyword("while");
  }
  is_bool() {
    return this.is_keyword("true") || this.is_keyword("false");
  }
  is_char() {
    return this.current.v[0] === "char";
  }
  is_str() {
    return this.current.v[0] === "str";
  }
  is_int() {
    return this.current.v[0] === "int";
  }
  is_float() {
    return this.current.v[0] === "float";
  }
  is_modifier() {
    const t = this.expect_plus() || this.expect_minus() && this.lookahead_n(2).v === ")";
    return this.is_open_paren() && t;
  }
  is_this() {
    return this.is_keyword("this");
  }
  is_keyword(t) {
    return this.current.v[0] === "key" && this.current.v[1] === t;
  }
  is_id() {
    return this.current.v[0] === "id";
  }
  is_id_pat() {
    return this.current.v[0] === "id";
  }
  is_bool_pat() {
    return this.current.v[0] === "bool";
  }
  is_char_pat() {
    return this.current.v[0] === "char";
  }
  is_str_pat() {
    return this.current.v[0] === "str";
  }
  is_int_pat() {
    return this.current.v[0] === "int";
  }
  is_float_pat() {
    return this.current.v[0] === "float";
  }
  is_tuple_pat() {
    return this.is_open_paren();
  }
  is_list_pat() {
    return this.is_open_bracket();
  }
  is_structl_pat() {
    return this.is_open_curly();
  }
  is_enum_pat() {
    return this.is_dot();
  }
  is_pat() {
    return this.is_bool_pat() || this.is_char_pat() || this.is_str_pat() || this.is_int_pat() || this.is_float_pat() || this.is_list_pat() || this.is_tuple_pat() || this.is_structl_pat() || this.is_enum_pat() || this.is_id_pat() || this.is_underscore();
  }
  is_assoc_fn() {
    if (g(this.current.v))
      return this.is_fn() && this.lookahead().v === "^";
  }
  is_method() {
    if (g(this.current.v))
      return this.is_fn() && this.lookahead().v !== "^";
  }
  expect_short_asgmt() {
    return this.lookahead().v === ":=";
  }
  expect_colon() {
    return this.lookahead().v === ":";
  }
  expect_comma() {
    return this.lookahead().v === ",";
  }
  expect_plus() {
    return this.lookahead().v === "+";
  }
  expect_minus() {
    return this.lookahead().v === "-";
  }
  expect_id() {
    return this.lookahead().v[0] === "id";
  }
  expect_str() {
    return this.lookahead().v[0] === "str";
  }
  expect_eof() {
    return this.lookahead().v === "$eof";
  }
  expect_close_paren() {
    return this.lookahead().v === ")";
  }
  expect_close_bracket() {
    return this.lookahead().v === "]";
  }
  expect_open_curly() {
    return this.lookahead().v === "{";
  }
  expect_close_curly() {
    return this.lookahead().v === "}";
  }
  expect_astrisk() {
    return this.lookahead().v === "*";
  }
  expect_asgmt() {
    return this.lookahead().v === "=";
  }
  maybe_asgmt() {
    if (this.is_asgmt())
      return this.next(), !0;
  }
  maybe_comma() {
    if (this.is_comma())
      return this.next(), !0;
  }
  optional_comma() {
    return this.is_newline() || this.is_close_curly() || this.is_close_paren() || this.is_close_bracket() || this.is_close_angle() || this.is_thin_arrow() || this.is_thick_arrow() ? this.maybe_comma() : this.req_comma();
  }
  maybe_colon() {
    if (this.is_colon())
      return this.next(), !0;
  }
  maybe_open_curly() {
    if (this.is_open_curly())
      return this.next(), !0;
  }
  maybe_id() {
    if (this.is_id()) {
      const t = this.current;
      return this.next(), t;
    }
  }
  maybe_asterisk() {
    if (this.is_asterisk())
      return this.current, this.next(), this.asterisk;
  }
  maybe_open_paren() {
    if (is_open_paren())
      return this.next(), !0;
  }
  maybe_modifier() {
    if (!this.is_modifier())
      return;
    this.next();
    const t = new h("modif", "", this.current);
    return this.next(), this.req_close_paren(), this.ast.push(t), !0;
  }
  maybe_attrs() {
    for (; this.is_hash(); ) {
      this.skip();
      const t = this.maybe_id();
      if (this.id || o("expecting an id: " + _(this.current)), this.attrs.push(t), this.lookahead().v !== ",")
        return null;
      this.skip();
    }
    if (this.attrs.length > 0)
      return !0;
  }
  maybe_pat() {
    if (this.is_pat())
      return this.prim_pat();
  }
  req_in() {
    if (this.is_in())
      return this.next(), !0;
    o('expecting "in" : ' + _(this.current));
  }
  req_asgmt() {
    return this.maybe_asgmt() || o("expecting '=' : " + _(this.current)), !0;
  }
  req_comma() {
    if (this.is_comma())
      return this.next(), !0;
    o("expecting ',' after : " + _(this.current));
  }
  req_backtick() {
    if (this.is_backtick())
      return this.next(), !0;
    o("expecting '`' after : " + _(this.current));
  }
  req_tbacktick() {
    if (this.is_tbacktick())
      return this.next(), !0;
    o("expecting '```' after : " + _(this.current));
  }
  req_terminator() {
    return this.is_newline() || this.is_eof();
  }
  req_open_paren() {
    if (this.is_open_paren())
      return this.next(), !0;
    o("expecting '(' : " + _(this.current));
  }
  req_close_paren() {
    if (this.is_close_paren())
      return this.next(), !0;
    o("expecting ')' : " + _(this.current));
  }
  req_open_curly() {
    if (this.is_open_curly())
      return this.next(), !0;
    o("expecting '{' : " + _(this.current));
  }
  req_close_curly() {
    if (this.is_close_curly())
      return this.next(), !0;
    o("expecting '}' : " + _(this.current));
  }
  req_close_angle() {
    if (this.is_close_angle())
      return this.next(), !0;
    o("expecting '>' : " + _(this.current));
  }
  req_colon() {
    if (this.is_colon())
      return this.next(), !0;
    o("expecting a colon ':' " + _(this.current));
  }
  req_dcolon() {
    if (this.is_dcolon())
      return this.next(), !0;
    o("expecting a double colon '::' " + _(this.current));
  }
  req_open_bracket() {
    if (this.is_open_bracket())
      return this.next(), !0;
    o("expecting '[' : " + _(this.current));
  }
  req_close_bracket() {
    if (this.is_close_bracket())
      return this.next(), !0;
    o("expecting ']' : " + _(this.current));
  }
  req_thin_arrow() {
    if (this.is_thin_arrow())
      return this.next(), !0;
    o("expecting '->' : " + _(this.current));
  }
  req_thick_arrow() {
    if (this.is_thick_arrow())
      return this.next(), !0;
    o("expecting '=>' : " + _(this.current));
  }
  req_then() {
    if (this.is_then())
      return this.next(), !0;
    o("expecting 'then' : " + _(this.current));
  }
  req_pat() {
    const t = this.maybe_pat();
    if (t)
      return t;
    o("expecting a pattern: " + _(this.current));
  }
  prim_pat() {
    if (this.is_underscore()) {
      const t = new h("_", "pat", "");
      return this.next(), t;
    } else {
      if (this.is_bool_pat())
        return this.bool_pat();
      if (this.is_char_pat())
        return this.char_pat();
      if (this.is_str_pat())
        return this.str_pat();
      if (this.is_int_pat())
        return this.int_pat();
      if (this.is_float_pat())
        return this.float_pat();
      if (this.is_list_pat())
        return this.list_pat();
      if (this.is_tuple_pat())
        return this.tuple_pat();
      if (this.is_structl_pat())
        return this.structl_pat();
      if (this.is_enum_pat())
        return this.enum_pat();
      if (this.is_id_pat())
        return this.id_pat();
    }
  }
  bool_pat() {
    const t = new h("bool", "pat", this.current);
    return this.next(), t;
  }
  char_pat() {
    const t = new h("char", "pat", this.current);
    return this.next(), t;
  }
  str_pat() {
    const t = new h("str", "pat", this.current);
    return this.next(), t;
  }
  int_pat() {
    const t = new h("int", "pat", this.current);
    return this.next(), t;
  }
  float_pat() {
    const t = new h("float", "pat", this.current);
    return this.next(), t;
  }
  list_pat() {
    if (!this.is_open_bracket())
      return;
    this.next();
    const t = [];
    for (this.maybe_comma(); ; ) {
      if (t.length > 0 && !this.req_comma())
        return;
      const s = this.maybe_pat();
      if (s)
        return t.push(s);
      break;
    }
    return this.maybe_comma(), this.req_close_bracket(), new h("[", "pat", t);
  }
  tuple_pat() {
    if (!this.is_open_paren())
      return;
    this.next();
    const t = [];
    for (this.maybe_comma(); ; ) {
      t.length > 0 && this.req_comma();
      const s = this.maybe_pat();
      if (s)
        return t.push(s);
      break;
    }
    return this.maybe_comma(), this.req_close_paren(), new h("(", "pat", t);
  }
  structl_pat() {
    if (!this.is_open_curly())
      return;
    this.next();
    const t = [];
    for (this.maybe_comma(); ; ) {
      t.length > 0 && (this.maybe_comma() || this.req_terminator());
      const s = this.maybe_id();
      if (!s)
        break;
      let i;
      this.is_colon() && (this.next(), i = this.req_pat());
      const n = new pt(s, i);
      t.push(n);
    }
    return this.maybe_comma(), this.req_close_curly(), new h("{", "pat", t);
  }
  enum_pat() {
    o("enum patterns are not supported yet");
    const t = this.current;
    this.next();
    const e = new ut(t, _pat);
    if (_pat)
      return new h("enum_pat", "pat", e);
  }
  id_pat() {
    const t = this.current;
    return this.next(), new h("id", "pat", t);
  }
  req_body() {
    const t = this.stmts();
    return new h("body", "body", t);
  }
  req_body_ret() {
    const t = this.req_body();
    return this.implicit_return(t.v), t;
  }
  maybe_stmt() {
    if (!(this.is_eof() || this.is_modifier()))
      return this.maybe_break() || this.maybe_const() || this.maybe_let() || this.maybe_expr() || this.maybe_semicolon();
  }
  req_stmts() {
    const t = this.current, e = this.stmts();
    if (N(e))
      o("expecting a statement : " + _(t));
    else
      return e;
  }
  stmts() {
    let t = [], e;
    for (; e = this.maybe_stmt(), e && t.push(e), !!e; )
      ;
    return t;
  }
  maybe_ret() {
    if (!this.is_ret())
      return;
    this.next();
    const t = this.maybe_expr();
    return new h("return", "expr", t);
  }
  maybe_break() {
    return this.is_break() ? (this.next(), new h("break", "stmt", null)) : void 0;
  }
  maybe_const() {
    if (!this.is_const())
      return;
    this.next();
    const t = this.req_pat(), e = this.maybe_tannotation();
    this.req_asgmt();
    const s = this.req_expr(), i = new J(t, e, s);
    return new h("const", "stmt", i);
  }
  maybe_let() {
    if (!this.is_let())
      return;
    this.next();
    const t = this.req_pat(), e = this.maybe_tannotation();
    this.req_asgmt();
    const s = this.req_expr(), i = new J(t, e, s);
    return new h("var", "stmt", i);
  }
  // maybe_var() {
  //     if(!this.is_var()) { return }
  //     this.next()
  //     const _pat = this.req_pat()
  //     const t = this.maybe_tannotation()
  //     let rhs
  //     if(this.maybe_asgmt()) { rhs = this.req_expr() }
  //     const v = new Asgmt(_pat, t, rhs)
  //     const n = new Node("var", "stmt", v)
  //     return n
  // }
  req_expr() {
    const t = this.current, e = this.maybe_expr();
    if (e)
      return e;
    o("expecting expression : " + _(t));
  }
  maybe_do_block_ret() {
    const t = this.maybe_do_block();
    if (t)
      return this.implicit_return(t.v), t;
  }
  req_do_block_ret() {
    const t = this.req_do_block();
    return this.implicit_return(t.v), t;
  }
  maybe_do_block() {
    if (!this.is_do())
      return;
    this.next();
    const t = [];
    for (this.req_open_curly(); !(this.is_eof() || this.is_close_curly()); ) {
      const s = this.maybe_stmt();
      if (s)
        t.push(s);
      else
        break;
    }
    return this.req_close_curly(), new h("do_block", "expr", t);
  }
  req_do_block() {
    const t = this.current, e = this.maybe_do_block();
    if (e)
      return e;
    o("expecting 'do' block : " + _(t));
  }
  maybe_block() {
    if (!this.is_open_curly())
      return;
    this.next();
    const t = [];
    for (; !(this.is_eof() || this.is_end()); ) {
      const s = this.maybe_stmt();
      if (s)
        t.push(s);
      else
        break;
    }
    return this.implicit_return(t), this.req_end(), new h("block", "expr", t);
  }
  maybe_semicolon() {
    if (!this.is_semicolon())
      return;
    const t = this.next();
    return new h(";", "expr", t);
  }
  is_bin_op() {
    if (this.current.v === "(" || this.current.v === "{" || this.current.v === "[") {
      if (!this.is_newline())
        return !0;
    } else
      return p(mt, this.current.v);
  }
  is_postfix_uni_op() {
    return p(kt, this.current.v);
  }
  req_list_index() {
    const t = this.req_expr();
    if (t) {
      if (this.req_close_bracket())
        return t;
    } else
      o("expecting  an index [...]: " + _(this.current));
  }
  req_access(t) {
    return this.req_expr();
  }
  req_call_args() {
    const t = [];
    for (; !(this.is_eof() || this.is_close_paren()); ) {
      t.length > 0 && (this.is_newline() ? this.maybe_comma() : this.req_comma());
      let s = this.maybe_expr();
      s && t.push(s);
    }
    return this.req_close_paren(), new h("args", "expr", t);
  }
  maybe_lopr_prefix_postfix(t, e) {
    const s = t.v.opr, i = t.v.op;
    if (t.id === "prefix")
      if (this.prec_uni(e) >= this.prec_uni(i)) {
        const n = new f(s, e), a = new h("postfix", "expr", n), u = new f(a, i), c = new h("prefix", "expr", u);
        return this.next(), c;
      } else {
        const n = new f(t, e), a = new h("postfix", "expr", n);
        return this.next(), a;
      }
  }
  maybe_lopr_bin_postfix(t, e) {
    if (t.id === "bin") {
      const s = t.v.op, i = t.v.ropr, n = t.v.lopr;
      if (this.prec_uni(e) >= this.prec_bin(s)) {
        const a = new f(i, e), u = new h("postfix", "expr", a), c = new A(n, s, u), l = new h("bin", "expr", c);
        return this.next(), l;
      } else {
        const a = new f(t, e), u = new h("postfix", "expr", a);
        return this.next(), u;
      }
    }
  }
  maybe_lopr_prefix_bin(t, e) {
    const s = t.v.op;
    let i = t.v.opr;
    if (t.id === "prefix")
      if (this.prec_uni(s) >= this.prec_bin(e)) {
        i = this.req_op(i);
        const n = new f(i, s);
        return new h("prefix", "expr", n);
      } else {
        const n = this.req_ropr(t), a = new A(i, e, n), u = new h("bin", "expr", a), c = new f(u, s);
        return new h("prefix", "expr", c);
      }
  }
  get_lopr(t, e) {
    let s = this.maybe_lopr_prefix_postfix(t, e);
    if (!s && (s = this.maybe_lopr_bin_postfix(t, e), !s)) {
      const i = new f(t, e);
      s = new h("postfix", "expr", i), this.next();
    }
    return s;
  }
  req_ropr(t) {
    let e;
    const s = this.current;
    return this.is_open_bracket() ? (this.next(), e = this.req_list_index()) : this.is_dot() ? (this.next(), e = this.req_access(t)) : this.is_open_paren() ? (this.next(), e = this.req_call_args()) : (this.next(), e = this.maybe_expr()), e || o("expecting right operand: " + _(s)), e;
  }
  prec_uni(t) {
    switch (t.v) {
      case "%":
        return 60;
      case ".":
        return 50;
      case "!":
      case "?":
        return 16;
      case "+":
      case "-":
      case "_!":
      case "not":
        return 15;
      case "⏎":
        return 0;
      default:
        o("unexpected unary operator: " + _(t));
    }
  }
  prec_bin(t) {
    switch (t.v) {
      case "[":
        return 20;
      case "(":
      case "{":
        return 19;
      case ".":
        return 18;
      case "*":
      case "×":
      case "/":
      case "÷":
        return 13;
      case "+":
      case "-":
        return 12;
      case "<":
      case "<=":
      case ">":
      case ">=":
        return 11;
      case "==":
      case "!=":
        return 10;
      case "<<":
      case ">>":
        return 8;
      case "&":
        return 7;
      case "**":
      case "⊕":
        return 6;
      case "|":
        return 5;
      case "&&":
        return 4;
      case "||":
        return 3;
      case "|>":
      case "||>":
      case ":>":
        return 2;
      case "=":
      case ":=":
      case "::":
      case "~=":
      case "+=":
      case "-=":
      case "*=":
      case "×=":
      case "/=":
      case "÷=":
      case "&=":
      case "|=":
      case "^=":
      case ">>=":
      case "<<=":
        return 1;
      default:
        o("unexpected binary operator: " + _(t));
    }
  }
  is_bin_rassoc(t) {
    return p(wt, t);
  }
  maybe_op(t) {
    const e = this.current;
    if (this.is_postfix_uni_op())
      return this.get_lopr(t, e);
    if (this.is_bin_op()) {
      const s = this.maybe_lopr_prefix_bin(t, e);
      if (s)
        return s;
      let i = this.req_ropr(t);
      i && (i.id === "bin" ? i = this.while_op(i, this.prec_bin(i.v.op) > this.prec_bin(e) || this.prec_bin(i.v.op) === this.prec_bin(e) && this.is_bin_rassoc(i.v.op)) : i.id === "prefix" ? i = this.while_op(i, this.prec_bin(e) > this.prec_uni(i.v.op) || this.is_bin_rassoc(e)) : i = this.while_op(i, !1));
      const n = new A(t, e, i);
      return new h("bin", "expr", n);
    }
  }
  req_op(t) {
    const e = this.maybe_op(t);
    if (e)
      return e;
    o("expect an operation: " + _(this.lookahead()));
  }
  while_op(t, e) {
    let s = m(t);
    if (this.is_eof())
      return s;
    for (; (this.is_bin_op() || this.is_postfix_uni_op()) && e !== !1; )
      s = this.req_op(s);
    return s;
  }
  maybe_expr() {
    let t = this.maybe_prim();
    if (t)
      return t = this.while_op(t), t;
  }
  req_id() {
    const t = this.maybe_id();
    if (t)
      return t;
    o("expecting an ID: " + _(this.current));
  }
  maybe_unit() {
    if (this.is_open_paren() && this.expect_close_paren())
      return this.next(), this.next(), new h("()", "expr", "()");
  }
  // FIXME: a workaround that only handles using id params
  maybe_afn(t) {
    const e = () => {
      if (!t)
        return;
      g(t) || (t = [t]);
      const s = [];
      return t.forEach((i) => {
        i.id = "id", i.type = "pat", s.push(new h("param", "pat", new D(i)));
      }), s;
    };
    if (this.is_thick_arrow()) {
      const s = e();
      this.next(), this.req_open_curly();
      const i = this.req_body_ret();
      this.req_close_curly();
      const n = new I("", s, null, i);
      return new h("afn", "expr", n);
    }
  }
  maybe_tuple_afn_group() {
    if (this.is_open_paren()) {
      const t = this.current.loc;
      if (this.next(), this.is_id() && this.expect_colon())
        return this.req_named_tuple();
      {
        let e = null;
        if (e = this.maybe_expr(), e) {
          let s = e;
          if (this.is_comma()) {
            const i = [s];
            for (; ; ) {
              const a = this.is_comma() && this.expect_close_paren();
              if (this.is_eof() || this.is_close_paren() || a)
                break;
              this.req_comma(), s = this.maybe_expr(), s ? i.push(s) : o("expected an argument: " + t);
            }
            this.maybe_comma(), this.req_close_paren();
            const n = this.maybe_afn(i);
            return n || new h("tuple", "expr", i);
          } else {
            this.req_close_paren();
            const i = this.maybe_afn(e);
            return i || (e.grouped = !0, e);
          }
        }
      }
    }
  }
  req_named_tuple() {
    let t = [];
    for (; !(this.is_eof() || this.is_close_paren()); ) {
      let e = this.req_id();
      this.req_colon();
      let s = this.req_expr();
      t.push([e, s]), this.optional_comma();
    }
    return this.req_close_paren(), new h("named_tuple", "expr", t);
  }
  is_prefix_uni_op() {
    return p(bt, this.current.v);
  }
  maybe_prefix_uni_op() {
    if (this.is_prefix_uni_op()) {
      const t = this.current;
      this.next();
      const e = new f(this.req_prim(), t);
      return new h("prefix", "expr", e);
    }
  }
  maybe_literal() {
    let t = this.maybe_primitivel();
    return t || (t = this.maybe_list()), t || (t = this.maybe_tuple()), t;
  }
  maybe_primitivel() {
    let t = this.maybe_bool();
    return t || (t = this.maybe_char()), t || (t = this.maybe_str()), t || (t = this.maybe_int()), t || (t = this.maybe_float()), t;
  }
  maybe_bool() {
    if (this.is_bool()) {
      const t = new h("bool", "expr", this.current);
      return this.next(), t;
    }
  }
  maybe_char() {
    if (this.is_char()) {
      const t = new h("char", "expr", this.current);
      return this.next(), t;
    }
  }
  maybe_str() {
    if (this.is_str()) {
      const t = new h("str", "expr", this.current);
      return this.next(), t;
    }
  }
  maybe_int() {
    if (this.is_int()) {
      const t = this.current;
      this.next();
      let e;
      return this.is_id() && !this.is_newline() && (e = this.current, this.next()), new h("int", "expr", [t, e]);
    }
  }
  maybe_float() {
    if (this.is_float()) {
      const t = this.current;
      this.next();
      let e;
      return this.is_id() && !this.is_newline() && (e = this.current, this.next()), new h("float", "expr", [t, e]);
    }
  }
  maybe_list() {
    if (this.is_open_bracket()) {
      const t = [];
      for (this.next(), this.maybe_comma(); ; ) {
        const s = this.is_comma() && this.expect_close_bracket();
        if (this.is_eof() || this.is_close_bracket() || s)
          break;
        t.length > 0 && (this.is_newline() ? this.maybe_comma() : this.req_comma()), t.push(this.req_expr());
      }
      return this.maybe_comma(), this.req_close_bracket(), new h("[", "expr", t);
    }
  }
  maybe_tuple() {
    if (this.is_open_paren()) {
      const t = [];
      this.skip(), this.maybe_comma();
      let e = !1;
      for (this.is_id && this.expect_colon() && (e = !0); ; ) {
        t.length > 0 && this.req_comma();
        let s;
        e && (s = this.req_id(), this.req_colon());
        let i = this.req_expr();
        if (e ? t.push([s, i]) : t.push([i]), this.is_close_paren())
          break;
      }
      return this.maybe_comma(), this.req_close_paren(), e ? new h("(:", "expr", t) : new h("(", "expr", t);
    }
  }
  maybe_call(t) {
    const e = () => {
      const u = [];
      if (this.is_open_paren()) {
        for (this.next(); !(this.is_eof() || this.is_close_paren()); ) {
          let c;
          this.is_id() && this.expect_colon() && (c = this.current, this.next(), this.next());
          let l;
          if (c ? l = new h("named_arg", "expr", [c, this.req_expr()]) : l = this.maybe_expr(), !l)
            break;
          u.push(l), this.optional_comma();
        }
        return this.req_close_paren(), u;
      }
    }, s = (u) => {
      if (!this.is_open_curly())
        return;
      this.next();
      const c = [];
      for (; !(this.is_eof() || this.is_close_curly()); ) {
        const l = this.maybe_expr();
        if (!l)
          break;
        c.push(l), this.optional_comma();
      }
      return this.req_close_curly(), c.push(), c;
    }, i = e(), n = s();
    return i || n ? new h("call", "expr", [t, i, n]) : void 0;
  }
  req_ref() {
    const t = this.current;
    return this.next(), new h("ref", "expr", t);
  }
  maybe_call_or_ref() {
    if (this.is_id()) {
      const t = this.req_ref();
      return this.maybe_call(t) || t;
    }
  }
  maybe_when_arm() {
    const t = [];
    for (; !this.is_thin_arrow() && (t.push(this.req_pat()), this.is_bar()); )
      this.next();
    this.req_thin_arrow();
    const e = this.req_expr(), s = new ft(t, e);
    return new h("arm", "", s);
  }
  maybe_when() {
    if (!this.is_when())
      return;
    this.next(), this.req_open_paren();
    const t = this.maybe_expr();
    this.req_close_paren(), this.req_open_curly();
    const e = [];
    for (this.maybe_comma(); !(this.is_eof() || this.is_close_curly() || e.length > 0 && (this.optional_comma(), this.is_eof() || this.is_close_curly())); ) {
      const n = this.maybe_when_arm();
      if (n)
        e.push(n);
      else
        break;
    }
    this.maybe_comma(), this.req_close_curly();
    const s = new dt(t, e);
    return new h("when", "expr", s);
  }
  maybe_prim() {
    let t = this.maybe_unit();
    return t || (t = this.maybe_tuple_afn_group()), t || (t = this.maybe_prefix_uni_op()), t || (t = this.maybe_call_or_ref()), t || (t = this.maybe_literal()), t || (t = this.maybe_when()), t || (t = this.maybe_do_block_ret()), t || (t = this.maybe_ret()), t || (t = this.maybe_semicolon()), t;
  }
  req_prim() {
    const t = this.maybe_prim();
    return t || o("expecting an expression: " + _(this.current)), t;
  }
  maybe_fn() {
    if (this.current, !this.is_fn())
      return;
    this.next();
    const t = this.req_fn();
    return t && p(["main", "بدء"], t.v.name.v[1]) ? new h("main", "fn", t.v) : new h("fn", "fn", t.v);
  }
  maybe_use() {
    if (!this.is_use())
      return;
    this.next();
    const t = [];
    if (!this.is_id())
      return !1;
    for (; this.is_id() && (t.push(this.current), this.next(), this.is_dot()); )
      this.skip(), this.is_id() || o("expecting an idnetifier after " + _(this.current));
    const e = new h("use", "stmt", t);
    return this.ast.push(e), !0;
  }
  maybe_optional() {
    if (this.is_question())
      return this.next(), !0;
  }
  maybe_type() {
    let t;
    if (this.is_open_bracket()) {
      this.next();
      const e = this.req_type();
      this.req_close_bracket();
      const s = new P(e, this.maybe_optional());
      t = new h("[", "t", s);
    } else if (this.is_open_curly()) {
      this.next();
      const e = [];
      for (this.maybe_comma(); !(this.is_eof() || this.is_close_curly()); ) {
        e.length > 0 && this.req_comma();
        const i = this.maybe_id();
        let n;
        i && this.is_colon() && (this.next(), n = this.req_type());
        const a = new z(i, n);
        e.push(a);
      }
      this.maybe_comma(), this.req_close_curly();
      const s = new _t(e, this.maybe_optional());
      t = new h("{", "t", s);
    } else if (this.is_id()) {
      const e = this.req_id();
      if (this.is_open_angle()) {
        const s = [];
        for (this.next(), this.maybe_comma(); !(this.is_eof() || this.is_close_angle()); )
          s.length > 0 && this.req_comma(), s.push(this.req_type());
        this.maybe_comma(), this.req_close_angle();
        const i = new lt(e, s, this.maybe_optional());
        t = new h("<", "t", i);
      } else {
        const s = new P(e, this.maybe_optional());
        t = new h("t", "t", s);
      }
    }
    return t;
  }
  req_type() {
    let t = this.maybe_type();
    return t || o("type required: " + _(this.current)), t;
  }
  maybe_tannotation() {
    if (this.is_colon())
      return this.next(), this.maybe_type();
  }
  req_tannotation() {
    const t = this.maybe_tannotation();
    return t || o("requires type annotation: " + _(this.current)), t;
  }
  maybe_fn_params() {
    const t = [];
    if (!this.is_open_paren())
      return t;
    for (this.next(), this.maybe_comma(); !(this.is_close_paren() || this.is_eof()); ) {
      t.length > 0 && this.req_comma();
      const e = this.req_pat(), s = this.maybe_tannotation(), i = new D(e, s), n = new h("param", "pat", i);
      e.id !== "id" && o("only parameters with id patterns are currently supported"), t.push(n);
    }
    return this.maybe_comma(), this.req_close_paren(), t;
  }
  maybe_fn_ret_types() {
    const t = [];
    if (this.is_thin_arrow())
      for (this.next(); this.is_id(); ) {
        let e = this.next();
        this.is_exclamation() ? (this.next(), t.push({
          id: e,
          _t: "throw"
        })) : t.push({
          id: e,
          _t: "ret_type"
        }), this.optional_comma();
      }
    return t;
  }
  req_fn() {
    const t = this.req_id();
    ot.push(t.v[1]);
    const e = this.maybe_fn_params(), s = this.maybe_fn_ret_types();
    if (this.is_open_curly()) {
      this.next();
      const i = this.req_body_ret(), n = new I(t, e, s, i), a = new h("fn", "fn", n);
      return this.req_close_curly(), a;
    } else {
      this.req_asgmt();
      const i = this.req_body_ret(), n = new I(t, e, s, i);
      return new h("fn", "fn", n);
    }
  }
  maybe_fields() {
    const t = [];
    for (; this.is_id(); ) {
      const e = new h("id", "expr", this.req_id()), s = this.req_tannotation(), i = new z(e, s), n = new h("field", "", i);
      if (t.push(n), this.is_close_paren())
        break;
      this.optional_comma();
    }
    return t;
  }
  maybe_typedef() {
    const t = () => {
      const c = [];
      if (this.is_open_paren()) {
        for (this.next(); !(this.is_eof() || this.is_close_paren()); ) {
          let l;
          this.is_id() && this.expect_colon() && (l = this.current, this.next(), this.next());
          let d;
          if (l ? d = new h("field", "expr", [l, this.req_type()]) : d = this.maybe_expr(), !d)
            break;
          c.push(d), this.optional_comma();
        }
        return this.req_close_paren(), c;
      }
    }, e = () => {
      if (!this.is_open_curly())
        return;
      this.next();
      const c = [];
      for (; !(this.is_eof() || this.is_close_curly()); ) {
        const l = this.maybe_expr();
        if (!l)
          break;
        c.push(l), this.optional_comma();
      }
      return this.req_close_curly(), c.push(), c;
    };
    if (!this.is_typedef())
      return;
    this.next();
    const s = this.req_id();
    U.push(s.v[1]);
    let i = t() || [];
    const n = e();
    if (!(i || n))
      return;
    if (n) {
      const c = { v: ["id", "__children"], loc: { line: 0, column: 0 } }, l = { id: "[", t: "t", v: { t: { id: "t", t: "t", v: { t: { v: ["id", "any"], loc: { line: 0, column: 0 } } } } } }, d = new h("field", "expr", [c, l]);
      i.push(d);
    }
    const a = new ct(s, i, n), u = new h("type", "def", a);
    return this.ast.push(u), !0;
  }
  maybe_consts_or_fns() {
    let t = !1;
    for (; ; )
      if (this.is_pat())
        this.ast.push(this.maybe_const()), t = !0;
      else if (this.is_fn())
        this.ast.push(this.maybe_fn()), t = !0;
      else
        break;
    return t;
  }
  implicit_return(t) {
    if (!t || t.length <= 0)
      return;
    const e = t.length - 1, s = t[e];
    if (!(p(["when", "while", "if", "for", "return", "let", "var", "const"], s.id) || s.id === "bin" && s.v.op.v === ":=") && s.id !== ";" && !(s.id === "bin" && s.v.op.v === "=") && !(s.id === "bin" && s.v.lopr.id === "ref" && p(["println", "اطبع_سطر"], s.v.lopr.v.v[1])) && s.t === "expr") {
      const i = new h("iret", "expr", s);
      return tt(t, e, i);
    }
  }
}
const xt = [
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
], B = [
  "readonly"
], yt = [
  "html",
  "body",
  "div",
  "span",
  "p",
  "textarea",
  "field"
];
function x(r) {
  return xt.includes(r) ? r.replaceAll("_", "-") : r;
}
const C = {
  صفحة_الشبكة: "html",
  راس: "head",
  نسق: "style",
  متن: "body",
  منطقة_النص: "textarea",
  عنوان_راسي٣: "h3",
  قسم: "div",
  سطر: "br"
}, W = (r) => {
  switch (r) {
    case "صنف":
      return "class";
    case "اعمدة":
      return "cols";
    case "صفوف":
      return "rows";
    case "للقراءة_فقط":
      return "readonly";
    default:
      return r;
  }
}, H = {
  // FIXME: workaround
  حوم: "hover"
}, gt = (r) => {
  switch (r) {
    case "عند":
      return "at";
    case "ازاحة_س":
      return "translateX";
    case "عنوان":
      return "url";
    default:
      return r;
  }
}, Y = (r) => {
  switch (r) {
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
      return r;
  }
}, T = (r) => {
  switch (r) {
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
      return r;
  }
}, E = (r) => {
  switch (r) {
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
      return r;
  }
}, qt = (r) => {
  switch (r) {
    case "متن":
      return "body";
    case "صفحة_الشبكة":
      return "html";
    default:
      return r;
  }
};
function F(r, t) {
  switch (r.id) {
    case "call":
      const e = r.v[0].v.v[1], s = C[e] || e, i = r.v[1] || [], n = r.v[2] || [];
      if (s === "br")
        return t += "<br>";
      switch (s) {
        case "اختر":
          t = X(i, t) + "} ";
          break;
        case "عرف_خط":
          t = $t(i, t);
          break;
        case "اطارات_رئيسية":
          t = jt(i, n, t);
          break;
        default:
          t += `<${s}`, yt.includes(s) && (t += " dir='rtl'"), i.forEach((a, u) => {
            if (u === 0 && a.id === "str")
              t += ` id='${a.v.v[1]}'`;
            else if (a.id === "named_arg") {
              const c = W(a.v[0].v[1]);
              if (B.includes(c))
                t += ` ${c} `;
              else if (t += ` ${c}= `, a.v[1].id === "str")
                t += `'${a.v[1].v.v[1]}'`;
              else if (a.v[1].id === "int" || a.v[1].id === "float") {
                const l = $(a.v[1].v[0].v[1]), d = a.v[1].v[1] && Y(a.v[1].v[1].v[1]) || "";
                t += `${l}${d}`;
              } else a.v[1].id === "bool" ? t += `${W(c)}` : o("not supported: " + _(a));
            } else
              o("not supported: " + _(a));
          }), t += ">", n.forEach((a) => {
            t = F(a, t);
          }), t += `</${s}>`;
      }
      break;
    case "str":
      t += r.v.v[1];
      break;
    default:
      o("unknown html element: " + _(r));
  }
  return t;
}
function X(r, t) {
  return r.forEach((e) => {
    const s = x(T(e.v[0].v[1])), i = e.v[1];
    s === "element" ? (t = Et(i, t), t += " {") : (t += `${s} : `, t = k(i, t), t += "; ");
  }), t;
}
function Et(r, t) {
  const e = (s) => {
    const i = (n) => RegExp(`(?<![p{L}\\p{N}_])${n}(?![\\p{L}\\p{N}_])`, "ug");
    return Object.keys(C).forEach((n) => {
      s = s.replaceAll(i(n), C[n]);
    }), Object.keys(H).forEach((n) => {
      s = s.replaceAll(i(n), H[n]);
    }), s;
  };
  if (g(r.v))
    t += " ", r.v.forEach((s, i) => {
      let n = s.v.v[1];
      t += e(n), i < r.v.length - 1 && (t += ",");
    });
  else {
    const s = r.v.v[1];
    t += e(s);
  }
  return t;
}
function k(r, t) {
  switch (r.id) {
    case "bool":
      o();
      break;
    case "int":
    case "float":
      const e = $(r.v[0].v[1]), s = Y(r.v[1] && r.v[1].v[1]) || "";
      t += e + s;
      break;
    case "prefix":
      t += r.v.op.v, t = k(r.v.opr, t);
      break;
    case "postfix":
      t = k(r.v.opr, t), t += r.v.op.v;
      break;
    case "str":
      t += qt(r.v.v[1]);
      break;
    case "ref":
      t += x(E(r.v.v[1]));
      break;
    case "tuple":
      r.v.forEach((c) => {
        t = k(c, t + " ");
      });
      break;
    case "call":
      const i = gt(r.v[0].v.v[1]), n = r.v[1];
      t += ` ${i}(`, n.forEach((c) => {
        t = k(c, t);
      }), t += ")";
      break;
    case "bin":
      const a = new Q();
      a.init(), a.write_expr(r);
      const u = a.get_code();
      t += "${" + u + "}".trim();
      break;
    default:
      o(`not supported: html generations:  ${_(r)}`);
  }
  return t;
}
function $t(r, t) {
  return t += "@font-face { ", t = X(r, t), t += "}", t;
}
function jt(r, t, e) {
  return e += ` @keyframes ${r[0].v.v[1]} { `, t && t.forEach((s) => {
    const i = s.v[0].v.v[1], n = E(s.v[1]);
    switch (i) {
      case "عند":
        const a = n[0], u = n[1].v || [];
        e = k(a, e), e += " {", u.forEach((c) => {
          if (c.id === "named_tuple")
            c.v.forEach((l) => {
              const d = x(T(l[0].v[1])), j = E(l[1]);
              e += ` ${d} : `, e = k(j, e), e += "; ";
            });
          else {
            const l = x(T(c[0].v[1])), d = E(c[1]);
            e += ` ${l} : `, e = k(d, e), e += "; ";
          }
        }), e += "} ";
        break;
      default:
        o("unsupported element: " + _(i));
    }
  }), e += "}", e;
}
function K(r, t) {
  switch (r.id) {
    case "call":
      let e = r.v[0].v.v[1], s = r.v[1] || [], i = r.v[2] || [];
      if (e === "br")
        return t += "<br>";
      switch (e) {
        case "select":
          t = St(s, t) + "} ";
          break;
        case "font_face":
          t = It(s, t);
          break;
        case "keyframes":
          t = Nt(s, i, t);
          break;
        default:
          t += `<${e}`, s.forEach((n, a) => {
            if (a === 0 && n.id === "str")
              t += ` id='${n.v.v[1]}'`;
            else if (n.id === "named_arg") {
              const u = n.v[0].v[1];
              if (B.includes(u))
                t += ` ${u} `;
              else if (t += ` ${u}= `, n.v[1].id === "str")
                t += `'${n.v[1].v.v[1]}'`;
              else if (n.v[1].id === "int" || n.v[1].id === "float") {
                const c = n.v[1].v[0].v[1], l = n.v[1].v[1].v[1] || "";
                t += `${c}${l}`;
              } else n.v[1].id === "bool" ? t += `${u}` : o("not supported: " + _(n));
            } else
              o("not supported: " + _(n));
          }), t += ">", i.forEach((n) => {
            t = K(n, t);
          }), t += `</${e}>`;
      }
      break;
    case "str":
      t += r.v.v[1];
      break;
    default:
      o("unknown html element: " + _(r));
  }
  return t;
}
function St(r, t) {
  return r.forEach((e) => {
    const s = x(e.v[0].v[1]), i = e.v[1];
    s === "element" ? (t = At(i, t), t += " {") : (t += `${s} : `, t = w(i, t), t += "; ");
  }), t;
}
function At(r, t) {
  return g(r.v) ? (t += " ", r.v.forEach((e, s) => {
    t += `${e.v.v[1]} `, s < r.v.length - 1 && (t += ",");
  })) : t += ` ${r.v.v[1]}`, t;
}
function w(r, t) {
  switch (r.id) {
    case "int":
    case "float":
      const e = r.v[0].v[1], s = r.v[1] && r.v[1].v[1] || "";
      t += e + s;
      break;
    case "prefix":
      t += r.v.op.v, t = w(r.v.opr, t);
      break;
    case "postfix":
      t = w(r.v.opr, t), t += r.v.op.v;
      break;
    case "str":
      t += r.v.v[1];
      break;
    case "ref":
      t += x(r.v.v[1]);
      break;
    case "tuple":
      r.v.forEach((c) => {
        t = w(c, t + " ");
      });
      break;
    case "call":
      const i = r.v[0].v.v[1], n = r.v[1];
      t += ` ${i}(`, n.forEach((c) => {
        t = w(c, t);
      }), t += ")";
      break;
    case "bin":
      const a = new JSGen();
      a.init(), a.write_expr(r);
      const u = a.get_code();
      pprint(u), t += "${" + u + "}".trim();
      break;
    default:
      o(`not supported: html generations:  ${_(r)}`);
  }
  return t;
}
function It(r, t) {
  return t += "@font-face { ", t = write_ar_css(r, t), t += "}", t;
}
function Nt(r, t, e) {
  return e += ` @keyframes ${r[0].v.v[1]} { `, t && t.forEach((s) => {
    const i = s.v[0].v.v[1], n = s.v[1];
    switch (i) {
      case "at":
        const a = n[0], u = n[1].v || [];
        e = w(a, e), e += " {", u.forEach((c) => {
          if (c.id === "named_tuple")
            c.v.forEach((l) => {
              const d = x(l[0].v[1]), j = l[1];
              e += ` ${d} : `, e = w(j, e), e += "; ";
            });
          else {
            const l = x(c[0].v[1]), d = c[1];
            e += ` ${l} : `, e = w(d, e), e += "; ";
          }
        }), e += "} ";
        break;
      default:
        o("unsupported element: " + _(i));
    }
  }), e += "}", e;
}
const Ct = `
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
    return س.__children
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
`, M = {
  بدء: "main",
  اطبع_سطر: "println",
  تعبير_نمطي: "regex",
  هذا: "this",
  مشيّد: "constructor",
  انهاء: "panic"
}, Tt = 4;
let Q = class {
  current;
  indent_level;
  stack;
  astructs;
  ast;
  main_args;
  opts;
  constructor(t, e, s, i, n, a) {
    return this.current = t, this.indent_level = e, this.stack = s, this.astructs = i, this.ast = n, this.opts = a, this;
  }
  init(t, e, s) {
    this.current = "", this.indent_level = 0, this.stack = [], this.astructs = [], this.ast = t, this.main_args = e, this.opts = s;
  }
  run() {
    this.strict_mode();
    let t, e = 0;
    for (; e < this.ast.length; ) {
      const i = this.ast[e];
      if (i) {
        const n = i.v;
        switch (i.id) {
          case "use":
            this.write_use(n);
            break;
          case "modif":
            this.write_modifier(n);
            break;
          case "main":
            t = n;
            break;
          case "const":
            this.write_const(n);
            break;
          case "fn":
            this.write_fn(n);
            break;
          case "type":
            this.write_typedef(n);
            break;
          default:
            o("unsupported node: " + this.ast[e].id);
        }
      }
      e += 1;
    }
    return this.write_helper_fns(), t && this.write_main(t), this.get_code();
  }
  to_en_id(t) {
    !t.v && !g(t.v) || M[t.v[1]] && (t.v[1] = M[t.v[1]]);
  }
  push() {
    this.stack.push(this.current), this.current = "";
  }
  pop() {
    this.current = this.stack.pop() + this.current;
  }
  pop_prepend() {
    this.current = this.current + this.stack.pop();
  }
  append(t) {
    this.current += t;
  }
  appendi(t) {
    this.current += this.spaces(), this.current += t;
  }
  spaces(t) {
    return t || (t = this.indent_level), et(" ", t * Tt);
  }
  strict_mode() {
    this.append(`"use strict";

`);
  }
  write_id_pat(t) {
    const e = t.v.v[1];
    this.append(e === "_" ? "default" : e);
  }
  write_char_pat(t) {
    this.append("'" + t.v.v[1] + "'");
  }
  write_str_pat(t) {
    this.append('"' + t.v.v[1] + '"');
  }
  write_tuple_pat(t) {
    this.append("(");
    let e = 0;
    for (; e < t.v.length; )
      this.write_pat(t.v[e]), e < t.v.length - 1 && this.append(", "), e += 1;
    this.append(")");
  }
  write_pat(t) {
    switch (t.id) {
      case "id":
        this.write_id_pat(t);
        break;
      case "bool":
        this.append(t.v.v[1]);
        break;
      case "int":
      case "float":
        this.append($(t.v.v[1][0]));
        break;
      case "char":
        this.write_char_pat(t);
        break;
      case "str":
        this.write_str_pat(t);
        break;
      case "tuple":
        this.write_tuple_pat(t);
        break;
      case "_":
        this.append("default");
        break;
      default:
        o("unsupported pattern " + _(t));
    }
  }
  write_modifier(t) {
    this.opts.ignore_export || t.v === "+" && this.appendi("export ");
  }
  write_use(t) {
  }
  write_main(t) {
    this.push(), this.appendi("("), this.write_fn(t, this.main_args), this.appendi(`)()
`), this.pop();
  }
  write_params(t) {
    this.append("(");
    let e = 0;
    for (; e < t.length; )
      e > 0 && this.append(", "), this.write_pat(t[e].v._pat), e += 1;
    this.append(")");
  }
  write_do_block(t) {
    this.append("(()=>"), this.write_block(t), this.append(`)() 
`);
  }
  write_block(t) {
    this.append(` {
`), this.push(), this.indent_level += 1;
    let e = 0;
    const s = t.v.length;
    for (; e < s; ) {
      const i = t.v[e];
      this.write_stmt(i), e += 1;
    }
    this.pop(), this.indent_level -= 1, this.appendi(`}
`);
  }
  write_fn(t, e) {
    this.push(), t.t === "fn" && this.appendi("static "), this.to_en_id(t.name), this.append("function " + t.name.v[1]), e ? this.append("()") : this.write_params(t.params), this.write_body(t.body, t.name === "main", e), this.pop();
  }
  write_fields(t) {
    const e = [];
    t.forEach((s) => {
      const i = s.v[0].v[1];
      e.push(i);
    }), e.forEach((s) => {
      this.appendi(this.spaces() + "" + s + `
`);
    }), this.write_init(e);
  }
  write_init(t) {
    this.append(`
`), this.appendi("constructor(");
    let e = 0;
    for (; e < t.length; )
      this.append(t[e]), e < t.length - 1 && this.append(", "), e += 1;
    for (this.append(`) {
`), this.indent_level += 1, e = 0; e < t.length; )
      this.appendi("this." + t[e] + " = " + t[e] + `
`), e += 1;
    this.appendi(`return this
`), this.indent_level -= 1, this.appendi(`}
`);
  }
  write_typedef(t) {
    this.appendi("class " + t.name.v[1] + ` {
`), this.indent_level += 1, t.fields && this.write_fields(t.fields), this.indent_level -= 1, this.appendi(`}

`);
  }
  write_const(t) {
    this.appendi("const "), this.write_pat(t.lhs), this.append(" = "), this.write_expr(t.rhs), this.append(`
`);
  }
  write_var(t) {
    this.appendi("let "), this.write_pat(t.lhs), t.rhs && (this.append(" = "), this.write_expr(t.rhs)), this.append(`
`);
  }
  write_ret(t) {
    this.append("return "), t.v && this.write_expr(t.v);
  }
  write_break(t) {
    this.append("break");
  }
  write_stmt(t) {
    t.t === "expr" ? (this.appendi(""), this.write_expr(t), this.append(`
`)) : t.id === "const" ? this.write_const(t.v) : t.id === "var" ? this.write_var(t.v) : t.id === "break" ? this.write_break(t) : o("cannot write stmt: " + _(t));
  }
  write_body(t, e, s) {
    if (this.append(` {
`), this.push(), this.indent_level += 1, s)
      for (const [a, u] of Object.entries(s))
        this.append(`const ${a} = '${u}'
`);
    let i = 0;
    const n = t.v.length;
    for (; i < n; ) {
      const a = t.v[i];
      this.write_stmt(a), i += 1;
    }
    this.pop(), this.indent_level -= 1, this.appendi(`}
`);
  }
  write_id(t) {
    this.append(t.v[1]);
  }
  write_ref(t) {
    const e = t.v.v[1];
    this.append(e);
  }
  write_str(t) {
    const e = t.v.v[1], s = e.indexOf("${") === -1 ? '"' : "`";
    this.append(s + e + s);
  }
  write_str_id(t) {
    this.append(symbol + t.v.v[1] + symbol);
  }
  is_call(t) {
    return t.v.id === "bin" && t.v.v.op.v === "(";
  }
  write_iret(t) {
    p(["when", "while", "if", "for", "return"], t.v.node) || t.v.t === "()" || this.is_call(t) || t.semicolon || this.append("return "), this.is_call(t) ? (this.append("const temp_seen_var = "), this.write_expr(t.v), this.append(`
`), this.append("return temp_seen_var")) : this.write_expr(t.v);
  }
  write_list(t) {
    this.append("[");
    let e = 0;
    const s = t.v.length;
    for (; e < s; )
      this.write_expr(t.v[e]), e < t.v.length - 1 && this.append(", "), e += 1;
    this.append("]");
  }
  write_structl(t) {
    const e = t.v;
    this.append("{");
    let s = 0;
    for (; s < e.length; ) {
      const i = e[s], n = i.k;
      n.v.v[1] ? this.write_id(n.v) : this.write_str_id(n.v);
      const a = i.v;
      this.append(": "), this.write_expr(a), s < e.length - 1 && this.append(", "), s += 1;
    }
    this.append("}");
  }
  write_args(t) {
    this.append("(");
    let e = 0;
    for (; e < t.v.length; ) {
      let s = t.v[e];
      s.v.op && s.v.op.v === ":" && (s = s.v.ropr), this.write_expr(s), e < t.v.length - 1 && this.append(", "), e += 1;
    }
    this.append(")");
  }
  write_named_arg(t) {
    t.v[0].v[1];
    const e = t.v[1];
    this.write_expr(e);
  }
  write_tuple(t) {
    this.append("[");
    let e = 0;
    for (; e < t.v.length; ) {
      let s = t.v[e];
      s.id === "narg" && (s = t.v[e].v[1]), this.write_expr(s), e < t.v.length - 1 && this.append(", "), e += 1;
    }
    this.append("]");
  }
  write_named_tuple(t) {
    this.append("{"), t.v.forEach((e, s) => {
      const i = e[0].v[1], n = e[1];
      this.append(i), this.append(": "), this.write_expr(n), s < t.v.length && this.append(",");
    }), this.append("}");
  }
  write_when(t) {
    this.appendi("switch("), this.write_expr(t.v.expr), this.append(`) {
`), this.indent_level += 1;
    let e = 0;
    for (; e < t.v.arms.length; ) {
      const s = t.v.arms[e], i = s.v.pats, n = s.v.expr;
      let a = 0;
      for (; a < i.length; )
        i[a].id !== "_" && this.appendi("case "), this.write_pat(i[a]), this.append(` :
`), a += 1;
      this.indent_level += 1, this.appendi(""), this.write_expr(n), this.append(`
`), this.appendi(`break
`), this.indent_level -= 1, e += 1;
    }
    this.indent_level -= 1, this.appendi(`}
`);
  }
  write_prefix_uni(t) {
    const e = t.v.op.v;
    switch (e) {
      case ".":
        if (t.v.opr.v.v[1] === "none") {
          this.append("null");
          return;
        } else
          o("enum variants are not supported, found : (." + t.v.opr.v.v[1] + ")");
        break;
      case "not":
        this.append("!");
        break;
      case "!":
      case "-":
        this.append(e);
        break;
      default:
        o("unsupported op: " + e);
        break;
    }
    this.write_expr(t.v.opr);
  }
  write_pipe(t) {
    for (; t.length > 0; ) {
      let e = t.pop();
      switch (e.id) {
        case "ref":
          this.write_expr(e), t.length > 0 && (this.append("("), this.write_pipe(t), this.append(")"));
          break;
        case "call":
          const s = e.v[0], i = e.v[1];
          this.write_expr(s), this.append("("), this.write_pipe(t), this.current.slice(-1) !== "(" && i.length > 0 && this.append(", "), i.forEach((n, a) => {
            this.write_expr(n), a < i.length - 1 && this.append(", ");
          }), this.append(")");
          break;
        case "int":
        case "float":
        case "str":
        case "[":
        case "tuple":
        case "named_tuple":
          this.write_expr(e);
          break;
        default:
          throw new Error("syntax error |> :" + _(e));
      }
    }
  }
  write_call(t) {
    if (this.to_en_id(t.v[0].v), t.v[0].v.v[1] === "html") {
      const s = K(t, "");
      this.append(` (() => \`${s}\`)() `);
      return;
    } else if (t.v[0].v.v[1] === "صفحة_الشبكة") {
      const s = F(t, "");
      this.append(`(() => \`${s}\`)()`);
      return;
    } else U.includes(t.v[0].v.v[1]) && this.append("new ");
    this.write_expr(t.v[0]), this.append("(");
    const e = t.v[1];
    e && e.forEach((s, i) => {
      this.write_expr(s), i < e.length - 1 && this.append(", ");
    }), t.v[2] && (e && this.append(", "), this.write_children(t.v[2])), this.append(")");
  }
  write_children(t) {
    !t || t.length === 0 || (this.append("["), t.forEach((e) => {
      this.write_expr(e), this.append(",");
    }), this.append("]"));
  }
  write_bin(t) {
    const e = t.v.op.v;
    switch (e) {
      case "[":
        this.write_expr(t.v.lopr), this.append("["), this.write_expr(t.v.ropr), this.append("]");
        break;
      case "=":
        this.write_expr(t.v.lopr), this.append("="), this.write_expr(t.v.ropr);
        break;
      case ":=":
        this.append("let "), this.write_expr(t.v.lopr), this.append(" = "), this.write_expr(t.v.ropr), this.append(`
`);
        break;
      case "::":
        this.appendi("const "), this.write_expr(t.v.lopr), this.append(" = "), this.write_expr(t.v.ropr), this.append(`
`);
        break;
      case ":":
        this.appendi("let "), this.write_expr(t.v.lopr), this.append(`
`);
        break;
      case "|>": {
        let s = [], i = t.v.lopr, n = t.v.ropr;
        for (; ; )
          if (s.push(i), n.id === "bin" && n.v.op.v === "|>")
            i = n.v.lopr, n = n.v.ropr;
          else {
            s.push(n);
            break;
          }
        this.write_pipe(s);
        break;
      }
      case "||>":
        throw new Error(" ||> : WIP , " + _(t));
      case ":>":
        throw new Error(" :> : WIP , " + _(t));
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
        this.write_expr(t.v.lopr), this.append(e), (e === "==" || e === "!=") && this.append("="), this.write_expr(t.v.ropr);
        break;
      default:
        o("cannot write binary operation: " + _(t));
        break;
    }
  }
  write_afn(t) {
    this.push(), this.write_params(t.v.params), this.append("=>"), this.write_body(t.v.body), this.pop();
  }
  write_expr(t) {
    switch (t.grouped && this.append("("), t.id) {
      case "()":
        break;
      case ";":
        break;
      case "ref":
        this.write_ref(t);
        break;
      case "bool":
        this.append(t.v.v[1]);
        break;
      case "int":
      case "float":
        this.append($(t.v[0].v[1]));
        break;
      case "char":
        this.append("'" + t.v.v[1] + "'");
        break;
      case "str":
        this.write_str(t);
        break;
      case "return":
        this.write_ret(t);
        break;
      case "iret":
        this.write_iret(t);
        break;
      case "[":
        this.write_list(t);
        break;
      case "{":
        this.write_structl(t);
        break;
      case "args":
        this.write_args(t);
        break;
      case "named_arg":
        this.write_named_arg(t);
        break;
      case "tuple":
        this.write_tuple(t);
        break;
      case "named_tuple":
        this.write_named_tuple(t);
        break;
      case "when":
        this.write_when(t);
        break;
      case "do_block":
        this.write_do_block(t);
        break;
      case "block":
        this.write_block(t);
        break;
      case "prefix":
        this.write_prefix_uni(t);
        break;
      case "call":
        this.write_call(t);
        break;
      case "bin":
        this.write_bin(t);
        break;
      case "afn":
        this.write_afn(t);
        break;
      default:
        o("cannot write expr: " + _(t));
    }
    t.grouped && this.append(")");
  }
  write_helper_fns() {
    this.append(Ct);
  }
  get_code() {
    return this.current;
  }
};
class Ot {
  ast;
  main_args;
  target;
  target_opts;
  init(t, e, s, i) {
    this.ast = t, this.main_args = e, this.target = s, this.target_opts = i;
  }
  run() {
    let t;
    switch (st(this.target)) {
      case "js":
        t = new Q();
        break;
      default:
        o('target "' + this.target + '" is not supported');
        break;
    }
    return t.init(this.ast, this.main_args, this.target_opts), t.run();
  }
}
class Lt {
  src;
  main_args;
  target;
  target_opts;
  lang;
  tokens;
  ast;
  gen_code;
  constructor(t, e, s, i, n, a, u, c, l, d) {
    return this.src = t, this.main_args = e, this.target = s, this.target_opts = i, this.lang = n, this.tokens = a, this.ast = u, this.symtab = c, this.transltab = l, this.gen_code = d, this;
  }
  init(t, e, s, i) {
    this.src = t, this.main_args = e, this.target = "js", this.target_opts = i || {}, this.lang = s || "en";
  }
  get_code() {
    return this.gen_code || this.compile(), this.gen_code;
  }
  compile() {
    this.scan(!0), this.parse(), this.generate(this.target);
  }
  scan(t) {
    const e = new ht();
    e.init(this.lang, this.src, t), e.run(), this.tokens = e.tokens, N(e.errs) || (O(e.errs), o(""));
  }
  parse() {
    const t = new vt();
    t.init(this.tokens), t.run(), this.ast = t.ast, N(t.errs) || (O(t.errs), o(""));
  }
  generate(t) {
    const e = new Ot();
    e.init(this.ast, this.main_args, t, this.target_opts), this.gen_code = e.run();
  }
}
export {
  Lt as default
};
