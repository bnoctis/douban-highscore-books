geckodriver &
curl -H 'Content-Type: application/json' -d '{"capabilities": {"alwaysMatch": {"browserName": "firefox"}}}' http://localhost:4444/session