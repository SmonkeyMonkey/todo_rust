# _Classical To Do list application_
 _This is todo application builded on rust,react for web app and react native for desktop application_



###  Installation
Clone repository:
```
git clone https://github.com/SmonkeyMonkey/todo_rust.git
```
### Usage
Move to web_app dir:
```
cd todo_rust/web_app
```
Run server:
```
docker-compose up -d
```
For create first user send POST request to http://localhost:8080/v1/user/create :
Example:
```json 
{
    "name": "test",
    "email": "testuser@gmail.com",
    "password": "secretpass"
}
```

For run front-end and desktop application change directory to front_end and run:
```
npm start
```

After creation our first user we can login http://localhost:8080/v1/auth/login

Example:
```json
{
    "username": "test",
    "password": "secretpass"
}
```
Note: if you are using postman, the login returns a token that you need to insert into the header in the 'token' field.

Or go to http://localhost:3000/login in browser or use desktop 
Note: front-end must be running


### Tests
Unit test:
```
sh scripts/run_unit_tests.sh
```

For run full tests we need to installed newman
```
npm install -g newman
```
Full test:
```
sh scripts/run_full_tests.sh
```
This run postman tests 
