#!/usr/bin/env python3
import json
import logging
import os
import signal
import sys
from http.server import BaseHTTPRequestHandler, HTTPServer
from socketserver import ThreadingMixIn

# 配置日志
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    handlers=[logging.StreamHandler(sys.stdout)],
)
logger = logging.getLogger("AnnouncementService")


class ThreadedHTTPServer(ThreadingMixIn, HTTPServer):
    """Handle requests in a separate thread."""

    pass


class AnnouncementHandler(BaseHTTPRequestHandler):
    def log_message(self, format, *args):
        """重写日志方法，使用我们配置的logger"""
        logger.info(f"{self.address_string()} - {format % args}")

    def do_GET(self):
        # Handle /get-info route
        if self.path == "/get-info":
            # Set response headers
            self.send_response(200)
            self.send_header("Content-type", "application/json")
            self.end_headers()

            # Read info.json file from root directory
            info_file = os.path.join(os.path.dirname(__file__), "info.json")

            if os.path.exists(info_file):
                with open(info_file, "r", encoding="utf-8") as file:
                    info_data = json.load(file)

                # Write the JSON response with ensure_ascii=False to prevent \u escaping
                self.wfile.write(
                    json.dumps(info_data, ensure_ascii=False).encode("utf-8")
                )
                logger.info("Served info.json content")
            else:
                self.send_response(404)
                self.end_headers()
                self.wfile.write(b"info.json not found")
                logger.warning("info.json not found")

        # Handle /get-announcement route
        elif self.path == "/get-announcement":
            # Set response headers
            self.send_response(200)
            self.send_header("Content-type", "application/json")
            self.end_headers()

            # Read files from the announces directory
            announces_dir = os.path.join(os.path.dirname(__file__), "announces")
            announcements = []

            if os.path.exists(announces_dir):
                for filename in os.listdir(announces_dir):
                    if filename.endswith(".md"):
                        filepath = os.path.join(announces_dir, filename)
                        with open(filepath, "r", encoding="utf-8") as file:
                            content = file.read()
                        announcements.append(
                            {"name": filename.replace(".md", ""), "content": content}
                        )

            # Write the JSON response with ensure_ascii=False to prevent \u escaping
            self.wfile.write(
                json.dumps(announcements, ensure_ascii=False).encode("utf-8")
            )
            logger.info(f"Served {len(announcements)} announcements")

        # Handle /get-latest-version route
        elif self.path == "/get-latest-version":
            # Set response headers
            self.send_response(200)
            self.send_header("Content-type", "application/json")
            self.end_headers()

            # Read info.json file
            info_file = os.path.join(os.path.dirname(__file__), "info.json")

            if os.path.exists(info_file):
                with open(info_file, "r", encoding="utf-8") as file:
                    info_data = json.load(file)

                # Write the JSON response with ensure_ascii=False to prevent \u escaping
                self.wfile.write(
                    json.dumps(info_data, ensure_ascii=False).encode("utf-8")
                )
                logger.info("Served latest version info")
            else:
                self.send_response(404)
                self.end_headers()
                self.wfile.write(b"info.json not found")
                logger.warning("info.json not found")

        else:
            # Handle 404 Not Found
            self.send_response(404)
            self.end_headers()
            self.wfile.write(b"Not Found")
            logger.warning(f"404 Not Found: {self.path}")


class AnnouncementService:
    def __init__(self, port=8080):
        self.port = port
        self.server = None
        self.running = False

    def start(self):
        """启动服务"""
        try:
            server_address = ("", self.port)
            self.server = ThreadedHTTPServer(server_address, AnnouncementHandler)
            self.running = True
            logger.info(f"Server starting on port {self.port}...")
            self.server.serve_forever()
        except Exception as e:
            logger.error(f"Server error: {e}")
            self.stop()

    def stop(self):
        """停止服务"""
        if self.server and self.running:
            logger.info("Shutting down server...")
            self.running = False
            self.server.shutdown()
            self.server.server_close()
            logger.info("Server stopped.")


# 全局服务实例
service = AnnouncementService()


def signal_handler(sig, frame):
    """信号处理函数"""
    logger.info(f"Received signal {sig}, shutting down...")
    service.stop()
    sys.exit(0)


if __name__ == "__main__":
    # 注册信号处理器
    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)

    logger.info("Announcement service starting...")
    service.start()
