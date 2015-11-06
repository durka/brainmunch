---
layout: page
title: brainmunch
---

<h3>Posts</h3>
<ul>
  {% for post in site.posts %}
    <li>
      <a href="{{ post.url }}">{{ post.title }}</a>
    </li>
  {% endfor %}
</ul>

---

This site was made with a template by [kbroman](http://github.com/kbroman/simple_site).

