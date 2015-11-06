---
layout: page
title: brainmunch
---

{% include JB/setup %}

<h3>Posts</h3>
<ul>
  {% for post in site.posts %}
    <li>
      <a href="{{ BASE_PATH }}{{ post.url }}">{{ post.title }}</a>
    </li>
  {% endfor %}
</ul>

---

This site was made with a template by [kbroman](http://github.com/kbroman/simple_site).

