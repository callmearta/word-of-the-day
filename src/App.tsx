import { useState, useEffect, useRef } from "react";
import { fetch, ResponseType } from "@tauri-apps/api/http";
import cheerio from "cheerio";
import "./App.css";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import Loading from "./components/Loading";

function App() {
  const [data, setData] = useState<{
    word?: string;
    pronounce?: string;
    desc?: string;
  }>({});
  const listenerRef = useRef<UnlistenFn>();

  const fetchData = async () => {
    try {
      const response = await fetch("https://dictionary.com/", {
        responseType: ResponseType.Text,
        method: "GET",
      });
      const html = response.data;

      // Load HTML content using Cheerio
      const $ = cheerio.load(html);

      // Extract link using Cheerio
      const link = $(
        'a[data-linklocation="body"][data-linkname="meanings-and-examples"]'
      ).attr("href");
      console.log(link);

      if (link) {
        const innerResponse = await fetch(link, {
          responseType: ResponseType.Text,
          method: "GET",
        });
        const innerHtml = innerResponse.data;
        const inner$ = cheerio.load(innerHtml);

        // Extract word, pronunciation, and description using Cheerio
        const word = inner$(".otd-item-headword__word h1").eq(0).text();
        const pronounce = inner$(".otd-item-headword__pronunciation__text")
          .eq(0)
          .text();
        const desc = inner$(".otd-item-headword__pos p:last-child")
          .eq(0)
          .text();

        setData({
          word,
          pronounce,
          desc,
        });
        invoke("update_title", {
          title: word,
        });
      }
    } catch (error) {
      console.error(error);
    }
  };

  const listenForDateChange = async () => {
    if (listenerRef.current) return;
    listenerRef.current = await listen("date-changed", () => {
      fetchData();
    });
  };

  useEffect(() => {
    if (listenerRef.current) listenerRef.current();

    listenForDateChange();

    return () => {
      if (listenerRef.current) listenerRef.current();
    };
  }, [listenerRef]);

  useEffect(() => {
    fetchData();
  }, []);

  return (
    <>
      <Loading visible={!data?.word} />
      <div className="word">
        <p>👑 Word of the day</p>
        <h1>{data?.word}</h1>
      </div>
      <div className="body">
        <p className="pronounce">{data?.pronounce}</p>
        <div className="desc">
          <p>{data?.desc}</p>
        </div>
      </div>
    </>
  );
}

export default App;
