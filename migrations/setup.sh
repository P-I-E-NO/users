source .env

node src/users.js down $TEST

node src/users.js up $TEST

echo "schema up"

if [ "$SEED" = "yes" ]; then

    echo "seeding"

fi

echo "all done"