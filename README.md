paperside-api
===

Rust / [Actix-Web](https://github.com/actix/actix-web) based API server for Project Paperside.

Uses [Diesel](https://github.com/diesel-rs/diesel) and Postgresql for database


##### Install

- Create development database server. Update the config in .env to match the database configuration

```
CREATE DATABASE paperside_api_development;
CREATE USER paperside_api_development WITH ENCRYPTED PASSWORD 'paperside_api_development';
GRANT ALL ON DATABASE paperside_api_development TO paperside_api_development;
```

- Initialize / migrate diesel models

```
diesel migration run
```


##### Test Setup

- Create test database server. Update the config in .env.test to match the testing database configuration

```
CREATE DATABASE paperside_api_test;
CREATE USER paperside_api_test WITH ENCRYPTED PASSWORD 'paperside_api_test';
GRANT ALL ON DATABASE paperside_api_test TO paperside_api_test;
```

##### Run Tests

- The test database will be reset and migrated by the test runner

```
cargo test
```


MIT License
---
Copyright (c) 2019 Mason Blier

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
