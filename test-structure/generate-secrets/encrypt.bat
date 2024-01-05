@echo off

echo Encrypting everything inside  %CD%/input folder.

docker run -it -e ACTION=encrypt --mount src=%CD%/input,target=/files_to_encrypt/,type=bind quay.sos.eu/edbafjdu/ansible-encrypt-decrypt
