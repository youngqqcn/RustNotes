#删除可执行文件
echo 当前目录是
pwd
echo 

read -r -p "是否删除可执行文件?"[y/n]  input
case $input in
    [yY][eE][sS]|[yY])
        ls ./ -F | grep '*' | sed 's/*//' | xargs rm -f
		echo "已删除所有可执行文件"
		;;
    [nN][oO]|[nN])
		echo "退出"
       	;;
    *)
		echo "Invalid input..."
		exit 1
		;;
esac

