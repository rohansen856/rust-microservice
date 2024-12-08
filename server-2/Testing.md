#POSTMAN TESTING
1.
- request url: `localhost:8080/todo`
- request mode: `POST`
- request body: 
```json
{
  "title": "Finish the report",
  "description": "Complete the quarterly sales report",
  "status": "pending",
  "due_date": "2024-10-30"
}
```
- response:
```json
{
    "id": 1,
    "title": "Finish the report",
    "description": "Complete the quarterly sales report",
    "status": "pending",
    "due_date": "2024-10-30",
    "created_at": "2024-10-23T13:34:15.763336",
    "updated_at": "2024-10-23T13:34:15.763336"
}
```

2.
- request url: `localhost:8080/todos`
- request mode: `GET`
- request body: ``
- response:
```json
[
    {
        "id": 1,
        "title": "Finish the report",
        "description": "Complete the quarterly sales report",
        "status": "pending",
        "due_date": "2024-10-30",
        "created_at": "2024-10-23T13:34:15.763336",
        "updated_at": "2024-10-23T13:34:15.763336"
    }
]
```
### Most common error
- Error:
```bash
Database(PgDatabaseError { severity: Error, code: "42P01", message: "relation \"todo\" does not exist", detail: None, hint: None, position: Some(Original(13)), where: None, schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("parse_relation.c"), line: Some(1449), routine: Some("parserOpenTable") })
```
- Reason: `Schema not pushed in DB, so table does not exist`
- Fix: Push the schema located in `todo.sql` to database