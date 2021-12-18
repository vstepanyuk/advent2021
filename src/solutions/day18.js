class Node {
  constructor(str) {
    this.str = str;
    this.value = parseInt(str);
  }

  isComma() {
    return (this.str = ",");
  }

  isOpen() {
    return this.str == "[";
  }

  isClose() {
    return this.str == "]";
  }

  isNumber() {
    return !isNaN(this.value);
  }

  static from(string) {
    return string.split("").map((ch) => new Node(ch));
  }

  static toString(nodes) {
    return nodes.map((n) => n.str).join("");
  }
}

function debug(stream) {
  for (let i = 0; i < stream.length; i++) {
    console.log(i, stream[i]);
  }
}

function explode(stream) {
  // debug(stream);
  let start = -1;
  let sum = 0;
  for (let i = 0; i < stream.length; i++) {
    let node = stream[i];
    if (node.isOpen()) {
      sum++;
    }

    if (node.isClose()) {
      sum--;
    }

    if (sum >= 5) {
      start = i;
      break;
    }
  }

  if (start == -1) {
    return stream;
  }

  let v1 = stream[start + 1].value;
  let v2 = stream[start + 3].value;

  let tmp = [new Node("0")];

  let added = false;
  let index = start - 1;
  while (index >= 0) {
    if (stream[index].isNumber() && !added) {
      tmp.unshift(new Node(`${stream[index].value + v1}`));
      added = true;
    } else {
      tmp.unshift(stream[index]);
    }
    index--;
  }

  index = start + 5;
  added = false;
  while (index < stream.length) {
    if (stream[index].isNumber() && !added) {
      tmp.push(new Node(`${stream[index].value + v2}`));
      added = true;
    } else {
      tmp.push(stream[index]);
    }
    index++;
  }

  return tmp;
}

function split(stream) {
  let tmp = [];
  let splitted = false;
  for (let i = 0; i < stream.length; i++) {
    if (!stream[i].isNumber() || splitted || stream[i].value < 10) {
      tmp.push(stream[i]);
      continue;
    }

    splitted = true;

    let a = Math.floor(stream[i].value / 2);
    let b = stream[i].value - a;

    tmp.push(
      new Node("["),
      new Node(`${a}`),
      new Node(`,`),
      new Node(`${b}`),
      new Node(`]`)
    );
  }

  return tmp;
}

function add(s1, s2) {
  return [new Node("[")]
    .concat(s1)
    .concat(new Node(","))
    .concat(s2)
    .concat(new Node("]"));
}

function reduce(node) {
  let i = 0;
  let last_s = Node.toString(node);
  let last = [...node];

  for (;;) {
    let last_s2 = Node.toString(last);
    for (;;) {
      let tmp = explode(last);
      let tmp_s = Node.toString(tmp);
      // console.log("EX:", tmp_s);

      if (tmp_s == last_s2) {
        break;
      }

      last = tmp;
      last_s2 = tmp_s;
    }

    let tmp = split(last);
    let tmp_s = Node.toString(tmp);

    // console.log("SPLIT:", tmp_s);

    if (tmp_s == last_s) {
      break;
    }

    last = tmp;
    last_s = tmp_s;
  }

  return last;
}

function magnitude(s) {
  const ex = s
    .replaceAll("[", "(")
    .replaceAll("]", ")")
    .replaceAll(",", "*3 + 2*");

  return eval(ex);
}

const data = require("fs").readFileSync(
  __dirname + "/../../inputs/day18.txt",
  "utf-8"
);

let lines = data.split("\n").map((l) => explode(Node.from(l.trim())));
let current = lines[0];

for (let i = 1; i < lines.length; i++) {
  current = add(current, lines[i]);
  current = reduce(current);
}

console.log("PART #1:", magnitude(Node.toString(current)));

const arr = [];
let max = 0;
for (let i = 0; i < lines.length; i++) {
  for (let j = 1; j < lines.length; j++) {
    let current = add(lines[i], lines[j]);
    current = reduce(current);

    arr.push(magnitude(Node.toString(current)));

    current = add(lines[j], lines[i]);
    current = reduce(current);

    let m = magnitude(Node.toString(current));
    if (m > max) {
      max = m;
    }
  }
}

console.log("PART #2:", max);
