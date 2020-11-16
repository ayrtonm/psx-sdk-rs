for i in */
    do cd $i;
    cargo fmt
    cd ..
done
