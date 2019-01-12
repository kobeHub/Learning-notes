#!/bin/sh 

while getopts "m:hn" arg  # 选项后的分号表示该选项需要参数
do 
  case $arg in
    m)
      echo The message of m is: $OPTARG  #参数存于OPTARG
      ;;
    h)
      echo "Get Help ./deploy -h:"
      echo "Usage:"
      echo "      ./deploy -m \"Addition message to git commit\""
      echo "      ./deploy -n Commit to git with no time stamp"
      ;;
    n)
      echo "Use -n"
      ;;
    ?)
      echo "Unknown argument"
  exit 1
  ;;
  esac
done
