use yew::prelude::*;
use crate::requests::news_api::{self, News, NewsArticle};


#[function_component]
pub fn HomeComponent() -> Html {
    let news_chunk = use_state(|| News::default());
    let news_chunk_setter = news_chunk.setter();
    let articles = (*news_chunk).clone().articles;
    
    use_effect_with((), move |_| news_api::get_news(news_chunk_setter));

    html! {
        <>  
            <link rel="stylesheet" href="/assets/css/home_style.css" />
            
            <div class={classes!("lumen-intro")}>
                <h3>{"Lumen - The Home Of Indie Game Developer!"}</h3>
            </div>

            <div style={"display: flex; justify-content: center;"}>
                <canvas id={"canvas"} width={"500px"} height= {"500px"}></canvas>
            </div>    
            <audio id={"audio"} controls={true}>
                <source src={"/assets/audio/ResidentEvilSaveRoomTheme.mp3"} type={"audio/mpeg"} />
            </audio>

            <div class={classes!("home-section")}>
                <h3>{"Guides"}</h3>
                <hr class={"line"}/>

                    <div class={"guide-images"}>
                        <div class={"image-container"}>
                            <img class={"guide-image"} src={"/assets/images/bevy.png"} />
                            <span>{"Bevy"}</span>
                        </div>
                        <div class={"image-container"}>
                            <img class={"guide-image"} src={"/assets/images/unreal.png"} />
                            <span>{"Unreal"}</span>
                        </div>
                        <div class={"image-container"}>
                            <img class={"guide-image"} src={"/assets/images/godot.webp"} />
                            <span>{"Godot"}</span>
                        </div>
                    </div>

                    <h3>{"Top 10"}</h3>
                    <hr class={"line"}/>
                    <div class={"top-images"}>
                        <div class={"top-image-container"}>
                            <img class={"top-image"} src={"/assets/images/thunder_chocolate_03.png"} />
                            <span>{"Thunder Chocolate"}</span>
                        </div>
                        <div class={"top-image-container"}>
                            <img class={"top-image"} src={"/assets/images/thunder_chocolate_04.png"} />
                            <span>{"Thunder Chocolate 2"}</span>
                        </div>
                        <div class={"top-image-container"}>
                            <img class={"top-image"} src={"/assets/images/woman_01.png"} />
                            <span>{"VR Weizenernte"}</span>
                        </div>
                        <div class={"top-image-container"}>
                            <img class={"top-image"} src={"/assets/images/woman_03.png"} />
                            <span>{"Call of Titties"}</span>
                        </div>
                        <div class={"top-image-container"}>
                            <img class={"top-image"} src={"/assets/images/woman_04.png"} />
                            <span>{"Call of Titties"}</span>
                        </div>
                        <div class={"top-image-container"}>
                            <img class={"top-image"} src={"/assets/images/thunder_chocolate_01.png"} />
                            <span>{"JoJo"}</span>
                        </div>
                        <div class={"top-image-container"}>
                            <img class={"top-image"} src={"/assets/images/thunder_chocolate_02.png"} />
                            <span>{"JoJo2"}</span>
                        </div>
                    </div>
                    <h3>{"News"}</h3>
                    <hr class={"line"}/>
                    <div class={"news-container"}>
                    {   
                        articles.iter().map(|article| {
                            html!{
                                <>  
                                <div class={"news-item"}>
                                    <a target={"_blank"} href={article.url.clone()}>
                                    <h4 key={article.clone().title.unwrap()}>
                                        {{article.title.clone().unwrap()}}
                                    </h4>
                                    <br/>
                                    <img src={article.urlToImage.clone()}/>
                                </a>
                                </div>
                                </>
                                }
                                }).collect::<Html>()
                    }
                    </div>
                </div>  
                <script src={"/assets/js/playAudio.js"}></script>
                <script src={"/assets/js/mageJumping.js"}></script>
        </>
    }
}
