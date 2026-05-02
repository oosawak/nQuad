#!/usr/bin/env python3
import http.server
import socketserver
import os

# Web ディレクトリで起動
web_dir = os.path.join(os.path.dirname(__file__), '..', 'web')
os.chdir(web_dir)

PORT = 8000
Handler = http.server.SimpleHTTPRequestHandler

class ReusableServer(socketserver.TCPServer):
    allow_reuse_address = True

print(f"🌐 Serving at http://localhost:{PORT}")
print(f"   Web root: {os.getcwd()}")
print("Press Ctrl+C to stop")

with ReusableServer(("", PORT), Handler) as httpd:
    httpd.serve_forever()



