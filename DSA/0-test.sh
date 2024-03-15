#!/bin/bash:wq

a=67
b=89

if [ "$a" -ne "$b" ]
then
	echo "$a is not equal to $b"
	echo "(arithmetic comparison)"

fi

echo "======> next stage"

if [ "$a"  != "$b" ]
then
	echo "$a is not equal to to $b"
	echo "(string comparison)"
fi
echo "=====> done"

let c=$a+$b
echo " ===> New value is $c"

let d=$c*8
echo "====> Train $d hours"

device="/dev/sda2"

if [ -b "$device" ]
then
	echo "$device is a block device."
fi

dev1="/dev/ttyS1"
if  [ -c "$dev1" ]
then
	echo "$dev1 is a character device"
fi
for ((i = 1; i <=10; i++))
do
	echo "Best School"
done

for i in {1..10}
do
	echo "ALX is Best School"
done

i=10

until [ $i == 1 ]
do
	echo "Best School"
done

i=10
until [ $i == 1 ]
do
	echo "Best School"
	i=$((i-1))
done

exit 0
