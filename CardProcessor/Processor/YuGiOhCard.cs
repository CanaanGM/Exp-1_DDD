using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace Processor
{
    public class YuGiOhCard
    {
        public int id { get; set; }
        public string? name { get; set; }
        public string? type { get; set; }
        public string? frameType { get; set; }
        public string? desc { get; set; }
        public string? race { get; set; }
        public string? archetype { get; set; }
        //public List<CardSet> card_sets { get; set; }
        public List<CardImage>? card_images { get; set; }
        //public List<CardPrice> card_prices { get; set; }
    }

    public class CardImage
    {
        public int id { get; set; }
        public string? image_url { get; set; }
        public string? image_url_small { get; set; }
        public string? image_url_cropped { get; set; }
    }

    //public class CardPrice
    //{
    //    public string cardmarket_price { get; set; }
    //    public string tcgplayer_price { get; set; }
    //    public string ebay_price { get; set; }
    //    public string amazon_price { get; set; }
    //    public string coolstuffinc_price { get; set; }
    //}

    //public class CardSet
    //{
    //    public string set_name { get; set; }
    //    public string set_code { get; set; }
    //    public string set_rarity { get; set; }
    //    public string set_rarity_code { get; set; }
    //    public string set_price { get; set; }
    //}

}
