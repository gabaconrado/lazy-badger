import os

USR_HOME_DIR = os.getenv('HOME')

APP_HOME = '.lazy-badger'
APP_SCRIPTS = 'scripts'
APP_BIN = 'bin'

LAZY_BADGER_HOME = os.path.join(USR_HOME_DIR, APP_HOME)
LAZY_BADGER_BIN = os.path.join(LAZY_BADGER_HOME, APP_BIN)

APP_PROXY_SCRIPT = '''#!/bin/bash

SCRIPT_NAME="$1.sh"
SCRIPT_TO_RUN={app_scripts_dir}/$SCRIPT_NAME
shift
chmod +x $SCRIPT_TO_RUN
$SCRIPT_TO_RUN "$@"

'''
