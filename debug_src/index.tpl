@extends: default.tpl
title: brainmunch
---

<ul>
    {% for post in posts %}
        <li><a href="posts/{{ post.name }}.html">{{ post.title }}</a></li>
    {% endfor %}
</ul>

