import sys
import json
from http.server import HTTPServer, BaseHTTPRequestHandler
from io import StringIO
import subprocess
import tempfile
import os
import shutil


class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        length = int(self.headers.get('Content-Length', 0))
        data = json.loads(self.rfile.read(length).decode())
        
        # Write code to a temporary file and run it inside Docker
        tmpdir = tempfile.mkdtemp()
        try:
            script_path = os.path.join(tmpdir, "main.py")
            with open(script_path, "w", encoding="utf-8") as f:
                f.write(data['code'])

            cmd = [
                "docker", "run", "--rm",
                "-v", f"{os.path.abspath(tmpdir)}:/code:ro",
                "python:3.11-slim",
                "python", "/code/main.py"
            ]

            proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
            output = proc.stdout or "No output"
        finally:
            shutil.rmtree(tmpdir, ignore_errors=True)

        
        self.send_response(200)
        self.end_headers()
        self.wfile.write(json.dumps({'result': output}).encode())

    def log_message(self, format, *args):
        pass

HTTPServer(('', 4000), Handler).serve_forever()