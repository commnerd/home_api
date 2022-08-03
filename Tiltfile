load('ext://uibutton', 'cmd_button')
# docker_compose("./docker-compose.yml")

app_resource_name = 'Home Api'

local_resource(app_resource_name, serve_cmd='./target/debug/home_api')

cmd_button('run-unit-tests', text="Unit Tests", resource=app_resource_name, argv=['/bin/bash', '-c', 'cargo test'])