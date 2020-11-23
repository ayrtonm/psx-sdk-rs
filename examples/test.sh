shopt -s extglob
echo "-----------" >> filesizes
for i in */
    do cd $i;
    cargo psx --release --no-pad | rg Text | awk '{print $3}' | xargs echo $i >> ../filesizes
    cd ..
done
echo "" >> filesizes
