---
layout: default
title: Home
---

# Rust Exercises

Welcome to the Rust exercises documentation. Below is a list of available exercises:

<ul>
{% for post in site.pages %}
  {% if post.layout == 'exercise' %}
    <li><a href="{{ post.url | relative_url }}">{{ post.title }}</a></li>
  {% endif %}
{% endfor %}
</ul> 