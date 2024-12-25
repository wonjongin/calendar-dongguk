import { useState, useEffect } from "react";
import { Calendar, dateFnsLocalizer } from "react-big-calendar";
import "react-big-calendar/lib/css/react-big-calendar.css";
import ICAL from "ical.js";
import axios from "axios";

import { format } from "date-fns/format";
import { parse } from "date-fns/parse";
import { startOfWeek } from "date-fns/startOfWeek";
import { getDay } from "date-fns/getDay";
import { ko } from "date-fns/locale/ko";

const locales = {
  ko: ko,
};

interface Event {
  title: string;
  start: Date;
  end: Date;
  description: string;
  location: string;
  backgroundColor: string;
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
  },
];
const generatePastelColor = () => {
  const hue = Math.floor(Math.random() * 360);
  return `hsl(${hue}, 70%, 80%)`;
};

export default function ICalendarComponent() {
  const [events, setEvents] = useState(sampleEvents);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  useEffect(() => {
    const fetchICalFile = async () => {
      setLoading(true);
      try {
        let url = "";
        if (import.meta.env.DEV) {
          console.log("개발 환경입니다");
          url = "http://localhost:3000/calendar/DONGGUK_2024.ics";
        } else if (import.meta.env.PROD) {
          url = "/calendar/DONGGUK_2024.ics";
        }

        const response = await axios.get(url, {
          responseType: "text",
        });

        const jcalData = ICAL.parse(response.data);
        const comp = new ICAL.Component(jcalData);
        const vevents = comp.getAllSubcomponents("vevent");

        const calendarEvents = vevents.map((vevent) => {
          const event = new ICAL.Event(vevent);
          return {
            title: event.summary,
            start: event.startDate.toJSDate(),
            end: event.endDate.toJSDate(),
            description: event.description,
            location: event.location,
            backgroundColor: generatePastelColor(),
          };
        });

        setEvents(calendarEvents);
      } catch (error) {
        console.error("Error fetching or parsing iCal file:", error);
        setError("일정을 불러오는데 실패했습니다.");
        setEvents(sampleEvents);
      } finally {
        setLoading(false);
      }
    };

    fetchICalFile();
  }, []);

  const handleEventClick = (event: Event) => {
    alert(`Event: ${event.title}\n\nDescription: ${event.description}`);
  };

  return (
    <div style={{ height: "800px", width: "1000px" }}>
      {error && (
        <div style={{ color: "red", marginBottom: "10px" }}>{error}</div>
      )}
      {loading && (
        <div style={{ color: "red", marginBottom: "10px" }}>로딩 중</div>
      )}
      <Calendar
        localizer={localizer}
        events={events}
        startAccessor="start"
        endAccessor="end"
        style={{ height: "100%" }}
        onSelectEvent={handleEventClick}
        views={["month", "week", "day", "agenda"]}
        defaultView="month"
        popup
        selectable
        messages={lang.ko}
        culture="ko"
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
    </div>
  );
}
