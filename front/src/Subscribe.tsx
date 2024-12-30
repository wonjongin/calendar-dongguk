import { useState, useEffect } from "react";

interface Univ {
  name: string;
  prefix: string;
  univ_code: string;
  years: Year[];
  crawler_type: string;
}

interface Year {
  year: number;
  year_code: string;
  url: string;
}

interface Category {
  id: number;
  name: string;
  keywords?: string[];
}

const categories: Category[] = [
  {
    id: 1,
    name: "수업",
    keywords: [
      "수강신청",
      "시험",
      "계절학기",
      "개강",
      "종강",
      "보강",
      "성적처리",
      "등록",
    ],
  },
  {
    id: 2,
    name: "학적",
    keywords: [
      "휴학",
      "복학",

      "전과",
      "다전공",
      "재입학",
      "전공신청",
      "학점포기",
    ],
  },
  {
    id: 128,
    name: "졸업",
    keywords: ["졸업", "졸업연기", "조기졸업", "졸업대상자"],
  },
  { id: 4, name: "대학원", keywords: ["대학원", "석사", "박사"] },
  { id: 8, name: "장학", keywords: ["장학"] },
  {
    id: 16,
    name: "교직",
    keywords: ["교직", "교육실습", "교육봉사", "교원", "학교현장실습"],
  },
  {
    id: 32,
    name: "행사",
    keywords: ["입학식", "학위수여식", "방학", "기념", "부처님"],
  },
  { id: 64, name: "기타" },
];

export default function Subscribe() {
  const [universities, setUniversities] = useState<Univ[]>([]);
  const [selectedUniv, setSelectedUniv] = useState("");
  const [selectedYear, setSelectedYear] = useState("");
  const [selectedCategories, setSelectedCategories] = useState<number[]>([]);

  useEffect(() => {
    let url = "";
    if (import.meta.env.DEV) {
      console.log("개발 환경입니다");
      url = "http://localhost:3000/univ.json";
    } else if (import.meta.env.PROD) {
      url = "/univ.json";
    }
    fetch(url)
      .then((response) => response.json())
      .then((data) => setUniversities(data))
      .catch((error) => console.error("Error loading universities:", error));
  }, []);

  const generateCategoryHash = () => {
    return selectedCategories.reduce((acc, curr) => acc + curr, 0);
  };

  const generateUrl = () => {
    const baseUrl = import.meta.env.DEV
      ? "http://localhost:3000"
      : "https://dgu-calendar.duckdns.org";
    const encoded = `${selectedUniv}_${selectedYear}_${generateCategoryHash()}`;
    return `${baseUrl}/c/${encoded}.ics`;
  };

  const handleCategoryChange = (categoryId: number) => {
    setSelectedCategories((prev) =>
      prev.includes(categoryId)
        ? prev.filter((id) => id !== categoryId)
        : [...prev, categoryId],
    );
  };

  const getSelectedUniversityYears = () => {
    const university = universities.find((u) => u.prefix === selectedUniv);
    return university?.years || [];
  };

  return (
    <div className="w-full sm:max-w-md mx-auto mt-0 sm:mt-10 p-4 sm:p-6 bg-white rounded-lg shadow-lg">
      <h1 className="text-lg sm:text-2xl font-bold mb-4 sm:mb-6 text-center">
        학사일정 캘린더 구독 링크 생성
      </h1>

      <div className="mb-4">
        <label className="block mb-2 text-sm sm:text-base">학교 선택</label>
        <select
          className="w-full p-2 border rounded text-sm sm:text-base"
          value={selectedUniv}
          onChange={(e) => {
            setSelectedUniv(e.target.value);
            setSelectedYear("");
          }}
        >
          <option value="">학교를 선택하세요</option>
          {universities.map((univ) => (
            <option key={univ.prefix} value={univ.prefix}>
              {univ.name}
            </option>
          ))}
        </select>
      </div>

      <div className="mb-4">
        <label className="block mb-2 text-sm sm:text-base">학년도 선택</label>
        <select
          className="w-full p-2 border rounded text-sm sm:text-base"
          value={selectedYear}
          onChange={(e) => setSelectedYear(e.target.value)}
          disabled={!selectedUniv}
        >
          <option value="">학년도를 선택하세요</option>
          {getSelectedUniversityYears().map((year) => (
            <option key={year.year} value={year.year}>
              {year.year}년
            </option>
          ))}
        </select>
      </div>

      <div className="mb-6">
        <label className="block mb-2 text-sm sm:text-base">
          카테고리 선택(중복가능)
        </label>
        <div className="space-y-2 text-sm sm:text-base">
          {categories.map((category) => (
            <label key={category.id} className="flex items-center">
              <input
                type="checkbox"
                checked={selectedCategories.includes(category.id)}
                onChange={() => handleCategoryChange(category.id)}
                className="mr-2"
              />
              {category.name}
              <span className="text-gray-500 text-[10px]">
                ({category.keywords?.join(", ")})
              </span>
            </label>
          ))}
        </div>
      </div>

      {selectedUniv && selectedYear && selectedCategories.length > 0 && (
        <div className="mt-4 sm:mt-6">
          <h2 className="font-bold mb-2 text-sm sm:text-base">구독 URL:</h2>
          <div className="p-2 sm:p-3 bg-gray-100 rounded break-all text-xs sm:text-base">
            {generateUrl()}
          </div>
          <button
            className="mt-3 sm:mt-4 w-full bg-blue-500 text-white p-2 rounded hover:bg-blue-600 text-sm sm:text-base"
            onClick={() => navigator.clipboard.writeText(generateUrl())}
          >
            URL 복사하기
          </button>
          <a
            className="mt-2 sm:mt-4 w-full bg-blue-500 text-white p-2 rounded hover:bg-blue-600 hover:text-white block text-center text-sm sm:text-base"
            href={generateUrl()}
          >
            캘린더에 추가하기
          </a>
        </div>
      )}
    </div>
  );
}
