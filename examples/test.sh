shopt -s extglob
echo "-----------" >> filesizes
for i in */
    do cd $i;
    cargo psx --no-pad --lto --size | rg Text | awk '{print $3}' | xargs echo $i >> ../filesizes
    cd ..
done
echo "" >> filesizes
