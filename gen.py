#!/usr/bin/env python
# -*- coding: utf-8 -*-
import json
import random


def gen_json(entries: int) -> str:
    return json.dumps([{"value": random.randint(0, 10)} for value in range(entries)])


def main() -> None:
    """Main function."""
    print(gen_json(10_000_000))


if __name__ == "__main__":
    main()
