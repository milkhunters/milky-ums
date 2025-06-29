#!/bin/sh

mc admin user add ums "$UMS_USER" "$UMS_PASSWORD"

mc admin policy create ums profile-photo ../policy/profile-photo-policy.json
mc admin policy attach ums profile-photo --user "$UMS_USER"
