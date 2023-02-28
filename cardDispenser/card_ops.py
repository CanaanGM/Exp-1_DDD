import pandas as pd


def load_cards() -> list[dict]:
    """Loads all cards in AllCards.JSON into a list

    Returns:
        list[dict]: list of all cards
    """
    cards: list = []
    all_cards = pd.read_json("../cardStoreFS/AllCardsRaw.json")
    for card in all_cards["data"]:
        cards.append(card)
    print(len(cards))
    return cards


CARDS: list[dict] = load_cards()
