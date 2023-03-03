use crate::YuGiOhCard;
use postgres::{Client, Error};

pub fn create_tables(client: &mut Client) -> Result<(), Error> {
    client.batch_execute(
        "
        BEGIN;
        CREATE TABLE public.yugiohcards (
            id int  NULL,
            \"name\" varchar NULL,
            \"type\" varchar NULL,
            frametype varchar NULL,
            \"desc\" varchar NULL,
            race varchar NULL,
            archetype varchar NULL
        );
            CREATE TABLE public.cardimages (
                id int  NULL,
                image_url varchar NULL,
                image_url_small varchar NULL,
                image_url_cropped varchar NULL
            );
            COMMIT;
        ",
    )?;
    Ok(())
}

pub fn insert_card_into_db(client: &mut Client, card: &YuGiOhCard) -> Result<(), Error> {
    // card should be broken down into -> card, card_images
    let card_to_insert = card;
    let card_images = &card.card_images;

    for card_image in card_images {
        client.execute(
            "
            INSERT INTO public.cardimages
            (id, image_url, image_url_small, image_url_cropped)
            VALUES($1, $2, $3, $4);
        ",
            &[
                &card.id,
                &card_image.image_url,
                &card_image.image_url_small,
                &card_image.image_url_cropped,
            ],
        )?;
    }

    client.execute(
        "
        INSERT INTO public.yugiohcards
        (id,\"name\",\"type\",frametype,\"desc\",race,archetype
        )
        VALUES($1, $2, $3, $4, $5, $6, $7);
    ",
        &[
            &card_to_insert.id,
            &card_to_insert.name,
            &card_to_insert.type_,
            &card_to_insert.frame_type,
            &card_to_insert.desc,
            &card_to_insert.race,
            &card_to_insert.archetype,
        ],
    )?;

    Ok(())
}

pub fn get_entries_count(client: &mut Client) -> Result<u64, Error> {
    let rows = client.query("SELECT COUNT(*) FROM public.yugiohcards", &[])?;
    let count: i64 = rows[0].get(0);
    Ok(count as u64)
}
