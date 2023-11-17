from persepolis.constants.status import CheckingFlag, ShutdownNotification

pyside6_is_installed = False

youtube_dl_is_installed = False

# CheckVersionsThread thread can change this variable.
ffmpeg_is_installed = True

shutdown_notification = ShutdownNotification.Running

checking_flag = CheckingFlag.Normal

# when rpc connection between persepolis and aria is disconnected >>
# aria2_disconnected = False >> every thing is ok :)
aria2_disconnected = False

# enum
aria_startup_answer = None

button_pressed_counter = 0

plugin_links_checked = False

temp_download_folder = ''
icons = ''
current_category_tree_index = None
connections = 0
download_path = ''
