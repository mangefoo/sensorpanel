#!/bin/bash

sudo cp sensorpanel.service /lib/systemd/system
sudo /lib/systemd/system/sensorpanel.service 
sudo systemctl enable sensorpanel.service
