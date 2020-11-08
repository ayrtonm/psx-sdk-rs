cd examples
echo "-----------" >> results
for i in */
    do cd $i;
    cargo b;
    cargo psx | rg Text | awk '{print $3}' | xargs echo $i >> ../results
    cd ..
done
echo "" >> results
