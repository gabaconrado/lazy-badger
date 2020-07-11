ACTIONS = {
    '--new': 'Create a new project',
    '--delete': 'Remove one project proxies'
}

def create_parser():
    '''
    Setup this app's arguments parser
    returns:
        (ArgumentParser): configured parser
    '''
    from argparse import ArgumentParser
    parser = ArgumentParser()
    actions = parser.add_mutually_exclusive_group()
    for key, value in ACTIONS.items():
        actions.add_argument(key, help=value, action="store_true")
    parser.add_argument('name', type=str, help='Project name')
    parser.add_argument('--path', type=str, help='App directory')
    return parser
