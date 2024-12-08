### Most common error
- Error:
```bash
Forwarding error: error sending request for url (http://localhost:8080/todos): error trying to connect: tcp connect error: Connection refused (os error 111)
```
- Reason: `Server url is defined in .env file, but server is actually not running`
- Fix: `Either run the server or remove the server URL from .env file`