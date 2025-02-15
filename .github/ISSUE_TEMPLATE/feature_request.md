---
name: Feature request
about: Suggest an idea for this project
title: ''
labels: ''
assignees: ''

---

name: Feature Request
description: Suggest a new feature or enhancement
title: "[Feature] "
labels: [enhancement]
assignees: 

body:
  - type: markdown
    attributes:
      value: |
        ## 🚀 Feature Request
        Describe the feature you want to add.

  - type: textarea
    attributes:
      label: Description
      description: Provide a clear and detailed explanation of the feature.
      placeholder: Describe the feature here...

  - type: textarea
    attributes:
      label: Tasks
      description: List the tasks needed to complete this feature.
      placeholder: |
        - [ ] Task 1
        - [ ] Task 2
        - [ ] Task 3
      render: markdown

  - type: dropdown
    attributes:
      label: Priority
      options:
        - High
        - Medium
        - Low

  - type: dropdown
    attributes:
      label: Related Milestone
      options:
        - MVP Release
        - Iteration 1
        - Iteration 2
        - Future Improvement
