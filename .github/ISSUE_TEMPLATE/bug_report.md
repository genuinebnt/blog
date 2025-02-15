---
name: Bug report
about: Create a report to help us improve
title: ''
labels: ''
assignees: ''

---

name: Bug Report
description: Report an issue with the application
title: "[Bug] "
labels: [bug]
assignees: 

body:
  - type: markdown
    attributes:
      value: "## 🐛 Bug Report"

  - type: textarea
    attributes:
      label: Describe the bug
      description: What happened? What did you expect?
      placeholder: A clear and concise description of the bug.

  - type: textarea
    attributes:
      label: Steps to reproduce
      description: How can we reproduce the issue?
      placeholder: |
        1. Go to '...'
        2. Click on '...'
        3. See error

  - type: textarea
    attributes:
      label: Expected behavior
      description: What should happen instead?
      placeholder: Describe the correct behavior.

  - type: textarea
    attributes:
      label: Screenshots or Logs
      description: If applicable, add screenshots or logs to help explain your problem.

  - type: dropdown
    attributes:
      label: Priority
      options:
        - High
        - Medium
        - Low
