import { useState, useEffect } from "react";
import { Calendar, dateFnsLocalizer } from "react-big-calendar";
import "react-big-calendar/lib/css/react-big-calendar.css";
import axios from "axios";

import { format } from "date-fns/format";
import { parse } from "date-fns/parse";
import { startOfWeek } from "date-fns/startOfWeek";
import { getDay } from "date-fns/getDay";
import { ko } from "date-fns/locale/ko";
import { categoryList } from "./Subscribe";

const locales = {
  ko: ko,
};

interface Event {
  title: string;
  start: Date;
  end: Date;
  description: string;
  location: string | undefined;
  category: string;
  backgroundColor: string;
}

interface JsonEvent {
  at: string[];
  title: string;
  org: string;
  category: string;
}

const localizer = dateFnsLocalizer({
  format,
  parse,
  startOfWeek,
  getDay,
  locales,
});

const lang = {
  ko: {
    week: "주",
    work_week: "주 (평일)",
    day: "일",
    month: "월",
    previous: "이전",
    next: "다음",
    today: "오늘",
    agenda: "일정",

    // showMore: (total) => `+${total} 더보기`,
  },
};

const sampleEvents = [
  {
    title: "겨울 휴가",
    start: new Date(2024, 11, 23), // 2024년 12월 23일
    end: new Date(2024, 11, 27), // 2024년 12월 27일
    description: "여행",
    location: "장소",
    backgroundColor: "#D85C06",
    category: "휴가",
  },
];
const generatePastelColor = () => {
  const hue = Math.floor(Math.random() * 360);
  return `hsl(${hue}, 70%, 80%)`;
};

export default function CalendarComponent() {
  const [events, setEvents] = useState<Event[]>(sampleEvents);
  const [filteredEvents, setFilteredEvents] = useState<Event[]>(sampleEvents);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [selectedCategories, setSelectedCategories] = useState<string[]>([
    "수업",
  ]);

  const handleCategoryClick = (category: string) => {
    console.log(selectedCategories);
    console.log(filteredEvents);
    setSelectedCategories((prev) => {
      if (prev.includes(category)) {
        return prev.filter((x) => x !== category);
      } else {
        return [...prev, category];
      }
    });
  };

  useEffect(() => {
    console.log("Sample Events:", events);
    console.log("Sample Event Category:", events[0]?.category);
    setFilteredEvents(
      events.filter((x) => selectedCategories.includes(x.category)),
    );
  }, [selectedCategories, events]);

  useEffect(() => {
    const fetchJsonFile = async () => {
      setLoading(true);
      try {
        let url = "";
        if (import.meta.env.DEV) {
          console.log("개발 환경입니다");
          url = "http://localhost:3000/j/DONGGUK/2024";
        } else if (import.meta.env.PROD) {
          url = "/j/DONGGUK/2024";
        }

        const response = await axios.get(url);
        const jsonData: JsonEvent[] = response.data;
        // console.log("Is Array?", Array.isArray(response.data));
        // const arrayData: JsonEvent[] = jsonData;
        // console.log(arrayData);
        // console.log("데이터 타입:", typeof arrayData);

        const calendarEvents = jsonData.map((event) => {
          if (event.at.length === 1) {
            event.at.push(event.at[0]);
          }
          return {
            title: event.title,
            start: new Date(event.at[0]),
            end: new Date(event.at[1]),
            description: event.org,
            backgroundColor: generatePastelColor(),
            category: event.category,
            location: undefined,
          };
        });
        console.log(calendarEvents);
        setEvents(calendarEvents);
      } catch (error) {
        console.error("Error fetching or parsing json file:", error);
        setError("일정을 불러오는데 실패했습니다.");
        setEvents(sampleEvents);
      } finally {
        setLoading(false);
      }
    };

    fetchJsonFile();
  }, []);

  const handleEventClick = (event: Event) => {
    alert(`Event: ${event.title}\n\nDescription: ${event.description}`);
  };

  return (
    <div style={{ width: "95vw" }}>
      <br />
      <br />
      {error && (
        <div style={{ color: "red", marginBottom: "10px" }}>{error}</div>
      )}
      {loading && (
        <div style={{ color: "red", marginBottom: "10px" }}>로딩 중</div>
      )}

      <Calendar
        localizer={localizer}
        events={filteredEvents}
        startAccessor="start"
        endAccessor="end"
        style={{ height: "800px" }}
        onSelectEvent={handleEventClick}
        views={["month", "week", "day", "agenda"]}
        defaultView="month"
        popup
        selectable
        messages={lang.ko}
        culture="ko"
        // showAllEvents={true}
        eventPropGetter={(event: Event) => ({
          style: {
            backgroundColor: event.backgroundColor,
            color: "white",
            borderRadius: "5px",
            border: "none",
            padding: "2px 5px",
            fontSize: "14px",
            fontWeight: "bold",
            // boxShadow: "0 2px 4px rgba(0,0,0,0.2)",
          },
        })}
      />
      <div>
        {categoryList.map((category) => (
          <label key={category.id} className="flex items-center">
            <input
              type="checkbox"
              checked={selectedCategories.includes(category.name)}
              onChange={() => handleCategoryClick(category.name)}
              className="mr-2"
            />
            {category.name}
            <span className="text-gray-500 dark:text-gray-400 text-[10px]">
              ({category.keywords?.join(", ")})
            </span>
          </label>
        ))}
      </div>
      <br />
      <br />
      <br />
      <br />
    </div>
  );
}
