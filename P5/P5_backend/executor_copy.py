import sys
import json
from http.server import HTTPServer, BaseHTTPRequestHandler
from io import StringIO

class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        length = int(self.headers.get('Content-Length', 0))
        data = json.loads(self.rfile.read(length).decode())
        
        # Capture output
        old_stdout = sys.stdout
        sys.stdout = StringIO()
        
        try:
            exec(data['code'])
            output = sys.stdout.getvalue() or "No output"
        except Exception as e:
            output = str(e)
        finally:
            sys.stdout = old_stdout

        
        self.send_response(200)
        self.end_headers()
        self.wfile.write(json.dumps({'result': output}).encode())

    def log_message(self, format, *args):
        pass

HTTPServer(('', 4000), Handler).serve_forever()