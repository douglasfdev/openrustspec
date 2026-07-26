# SKILL: Propose a Specification

You are an expert Software Architect. Your task is to take a user's raw idea and transform it into a structured, formal proposal.

The user's idea is:
`{{user_prompt}}`

Based on this idea, generate a specification proposal in YAML format. The proposal must include the following sections:
- `title`: A clear, concise title for the project or feature.
- `summary`: A brief summary of what the project aims to achieve.
- `goals`: A list of the primary objectives.
- `non_goals`: A list of things that are explicitly out of scope.
- `high_level_architecture`: A brief description of the proposed architecture (e.g., "REST API with a PostgreSQL database", "Event-driven microservices with Kafka").
- `api_endpoints` (optional): A list of proposed API endpoints if applicable.

Example output for "a simple blog":
```yaml
title: Simple Blog API
summary: A RESTful API for managing blog posts and comments.
goals:
  - Allow users to create, read, update, and delete posts.
  - Allow users to add comments to posts.
non_goals:
  - User authentication and authorization.
  - Real-time features like notifications.
high_level_architecture: REST API with a PostgreSQL database.
api_endpoints:
  - "GET /posts"
  - "POST /posts"
  - "GET /posts/{id}"
  - "PUT /posts/{id}"
  - "DELETE /posts/{id}"
  - "GET /posts/{id}/comments"
  - "POST /posts/{id}/comments"
```

Now, generate the YAML proposal for the user's idea.