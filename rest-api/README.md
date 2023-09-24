Get All:
-------
curl -s http://localhost:8080/api/todos | jq

Create:
-------
curl -X POST -H "Content-Type: application/json" -d '{"title": "My first todo", "description": "This is my first todo"}' http://localhost:8080/api/todos
curl -X POST -H "Content-Type: application/json" -d '{"title": "Buy milk", "description": "Buy 2 liters of milk"}' http://localhost:8080/api/todos

Get by id:
----------
curl -s http://localhost:8080/api/todos/590538de-56c4-4057-b4e6-c91021fc04be | jq

Update:
-------
curl -s -X PUT -H "Content-Type: application/json" -d '{"title": "Buy 2 liters of milk", "description": "Buy 2 liters of milk"}' http://localhost:8080/api/todos/b6b8b5ef-3b61-4b96-8eaf-c2ccaca152b8

Delete:
-------
curl -s -X DELETE http://localhost:8080/api/todos/b6b8b5ef-3b61-4b96-8eaf-c2ccaca152b8 | jq
