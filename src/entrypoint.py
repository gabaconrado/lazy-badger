#!/usr/bin/python
import sys

import app
from parser import create_parser, ACTIONS

REQUIRED_FIELDS = ['path']

if __name__ == '__main__':
    app.setup_directories()
    parser = create_parser()
    args = parser.parse_args()
    if (args.new):
        if not all([getattr(args, field) for field in REQUIRED_FIELDS]):
            sys.exit(f'"--new" must specify all following params: {",".join(REQUIRED_FIELDS)}')
        app.create_app_home_and_scripts(args.name)
        app.create_app_proxy_executable(args.name, args.path)
    elif (args.delete):
        app.uninstall_app(args.name)
