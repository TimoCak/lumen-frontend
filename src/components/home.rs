use yew::prelude::*;
use crate::{apis::news_api, models::news::News};
use crate::apis::news_api::NEWS_OFFSET_START;


#[function_component]
pub fn HomeComponent() -> Html {
    let news_chunk = use_state(|| News::default());
    let news_chunk_setter = news_chunk.setter();
    let news_offset: UseStateHandle<usize> = use_state(|| NEWS_OFFSET_START);
    let articles = (*news_chunk).clone().articles;


    use_effect_with(news_offset.clone(), move |offset| news_api::get_news(news_chunk_setter, (*offset.to_owned())));

    let increment_offset = Callback::from(move |_| {
        let news_offset_new = (*news_offset).clone() + NEWS_OFFSET_START;

        news_offset.setter().clone().set(news_offset_new);
    });

    html! {
        <>  
            <link rel="stylesheet" href="/assets/css/home_style.css" />
            
            <div class={classes!("lumen-intro")}>
                <h3>{"Lumen - The Home Of Indie Game Developer!"}</h3>
            </div>

                

            <div class={classes!("home-section")}>
                <h3>{"Guides"}</h3>
                <hr class={"line"}/>

                    <div class={"guide-images"}>
                        <div class={"image-container"}>
                            <a href={"https://bevyengine.org/"} target={"_blank"}>
                                <img class={"guide-image"} src={"/assets/images/bevy.png"} />
                            </a>    
                            <span>{"Bevy"}</span>
                        </div>
                        <div class={"image-container"}>
                            <a href={"https://www.unrealengine.com/en-US/learn"} target="_blank">
                                <img id={"unreal-image"} class={"guide-image"} src={"/assets/images/unreal.png"} />
                            </a>
                        <span>{"Unreal"}</span>    
                        </div>
                        <div class={"image-container"}>
                            <a href={"https://godotengine.org/"} target={"_blank"}>
                                <img class={"guide-image"} src={"/assets/images/godot.webp"} />
                            </a>
                        <span>{"Godot"}</span>    
                        </div> 
                        <div class={"image-container"}>
                            <a href={"https://docs.unity.com/"} target={"_blank"}>
                                <img class={"guide-image"} src={"/assets/images/unity.png"} />    
                            </a>
                            <span>{"Unity"}</span>
                        </div>
                    </div>

                    <h3>{"Top Rated Showcases"}</h3>
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

                    <h3>{"Canvas Games"}</h3>
                    <hr class={"line"}/>
                    <div class={"canvas-games"}>
                        <canvas id={"canvas"} width={"500px"} height= {"500px"}></canvas>
                        <canvas id={"snake-canvas"} width={"500px"} height= {"500px"}></canvas>
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
                    <button onclick={increment_offset}>{"load more news"}</button>
                </div>
                <script src={"/assets/js/mageVsZombies.js"}></script>
                <script src={"/assets/js/snake.js"}></script>
        </>
    }
}
