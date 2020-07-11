import os
import constants as c

def setup_directories():
    '''
    Generate the app's structure
    '''
    os.makedirs(c.LAZY_BADGER_HOME, exist_ok=True)
    os.makedirs(c.LAZY_BADGER_BIN, exist_ok=True)


def _get_app_home(app_name):
    '''
    Get the home directory from an app
    args:
        (str) app_name: Name of the app
    '''
    return os.path.join(c.LAZY_BADGER_HOME, app_name)


def _get_app_scripts(app_name):
    '''
    Get the scripts directory for an app
    args:
        (str) app_name: Name of the app
    '''
    return os.path.join(_get_app_home(app_name), c.APP_SCRIPTS)


def create_app_home_and_scripts(app_name):
    '''
    Generate a new app scripts folder
    args:
        (str) app_name: Name of the new app
    '''
    os.makedirs(_get_app_scripts(app_name), exist_ok=True)


def create_app_proxy_executable(app_name, app_path):
    '''
    Generate a new proxy executable for a new app
    args:
        (str) app_name: Name of the app
        (str) app_path: Workdir of the app
    '''
    from stat import S_IXGRP
    exec_path = os.path.join(c.LAZY_BADGER_BIN, app_name)
    with open(exec_path, 'w') as app_proxy:
        app_proxy.write(
            c.APP_PROXY_SCRIPT.format(
                workdir=app_path,
                app_scripts_dir=_get_app_scripts(app_name)
            )
        )
    os.chmod(exec_path, 0o755)


def uninstall_app(app_name):
    '''
    Uninstall an app
    args:
        (str) app_name: Name of the app
    '''
    from shutil import rmtree
    rmtree(_get_app_home(app_name), ignore_errors=True)
    os.remove(os.path.join(c.LAZY_BADGER_BIN, app_name))
