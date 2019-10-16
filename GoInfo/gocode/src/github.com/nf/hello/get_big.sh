#!/bin/bash
ALL="0123456789abcdefghjklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ~!@#$%^&*()_+"
LENGTH="15"
#while [ "${i:=1}" -le 10 ]
#do 
#  while [ "${n:=1}" -le "$LENGTH" ]
#  do
#    string="${string}${ALL:$(($RANDOM%${#ALL})):1}"
#    let n+=1
#  done
#  echo "$string"
#  let i+=1
#done
function getbase() {
  for i in {1..10000}
  do
    strs=`openssl rand -base64 15`
    echo "$strs" >> $1
  done
}

function getfile() {
  for i in {1..10000}
  do
    cat $1 >> $2
  done
}

getfile $1 $2
