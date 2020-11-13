echo "-----------" >> results
for i in */
    do cd $i;
    cargo psx --release | rg Text | awk '{print $3}' | xargs echo $i >> ../results
    cd ..
done
echo "" >> results
