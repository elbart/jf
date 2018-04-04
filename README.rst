==============
Json Flattener
==============

Flattens JSON structures to make them easily greppable.

Example Usage
=============
Currently, ``jf`` only reads from `stdin`::

    $> echo '{"hello": {"huhu" : "foo", "bla": [1, "asd", null, false, {"tim": "recursive"}]}}' | jf
    .hello = {};
    .hello.bla = [];
    .hello.bla[0] = 1;
    .hello.bla[1] = "asd";
    .hello.bla[2] = null;
    .hello.bla[3] = false;
    .hello.bla[4] = {};
    .hello.bla[4].tim = "recursive";
    .hello.huhu = "foo";

TODO
====
- read from files, not only stdin
- make output fully JS compliant
- improve performance
