#!/usr/bin/python

import app
from parser import create_parser, ACTIONS


if __name__ == '__main__':
    app.setup_directories()
    parser = create_parser()
    args = parser.parse_args()
    if (args.new):
        app.create_app_home_and_scripts(args.name)
        app.create_app_proxy_executable(args.name)
    elif (args.delete):
        app.uninstall_app(args.name)
