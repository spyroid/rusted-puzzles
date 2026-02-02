#!/usr/bin/env python3

import re
import sys


def gen_regex(r="0", depth=15):
    if depth == 0:
        return ""
    if rules[r][0][0].startswith('"'):
        return rules[r][0][0].strip('"')
    return "(" + "|".join(["".join([gen_regex(sub, depth - 1) for sub in subrule]) for subrule in rules[r]]) + ")"


with open('input.txt', "r") as f:
    rules_raw, msgs = f.read().split("\n\n")

rules = {}
for rule in rules_raw.split("\n"):
    num, r = rule.split(": ")
    rules[num] = [s.split() for s in r.split(" | ")]

res = gen_regex()
print(res)

r1 = re.compile(res)
res = [r1.fullmatch(msg) for msg in msgs.split("\n")]
print(f"Part 1: {len([x for x in res if x])}")

rules["8"] = [["42"], ["42", "8"]]
rules["11"] = [["42", "31"], ["42", "11", "31"]]

r2 = re.compile(gen_regex())
res = [r2.fullmatch(msg) for msg in msgs.split("\n")]
print(f"Part 2: {len([x for x in res if x])}")