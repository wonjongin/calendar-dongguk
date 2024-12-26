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
}

const categories: Category[] = [
  { id: 1, name: "수업" },
  { id: 2, name: "학적" },
  { id: 4, name: "대학원" },
  { id: 8, name: "장학" },
  { id: 16, name: "교직" },
  { id: 32, name: "행사" },
  { id: 64, name: "기타" },
];

export default function Subscribe() {
  const [universities, setUniversities] = useState<Univ[]>([]);
  const [selectedSchool, setSelectedSchool] = useState("");
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
    const encoded = `${selectedSchool}_${selectedYear}_${generateCategoryHash()}`;
    return `${baseUrl}/c/${encoded}/cal.ics`;
  };

  const handleCategoryChange = (categoryId: number) => {
    setSelectedCategories((prev) =>
      prev.includes(categoryId)
        ? prev.filter((id) => id !== categoryId)
        : [...prev, categoryId],
    );
  };

  const getSelectedUniversityYears = () => {
    const university = universities.find((u) => u.univ_code === selectedSchool);
    return university?.years || [];
  };

  return (
    <div className="max-w-md mx-auto mt-10 p-6 bg-white rounded-lg shadow-lg">
      <h1 className="text-2xl font-bold mb-6">
        학사일정 캘린더 구독 링크 생성
      </h1>

      <div className="mb-4">
        <label className="block mb-2">학교 선택</label>
        <select
          className="w-full p-2 border rounded"
          value={selectedSchool}
          onChange={(e) => {
            setSelectedSchool(e.target.value);
            setSelectedYear(""); // 학교가 변경되면 년도 선택 초기화
          }}
        >
          <option value="">학교를 선택하세요</option>
          {universities.map((univ) => (
            <option key={univ.univ_code} value={univ.univ_code}>
              {univ.name}
            </option>
          ))}
        </select>
      </div>

      <div className="mb-4">
        <label className="block mb-2">학년도 선택</label>
        <select
          className="w-full p-2 border rounded"
          value={selectedYear}
          onChange={(e) => setSelectedYear(e.target.value)}
          disabled={!selectedSchool}
        >
          <option value="">학년도를 선택하세요</option>
          {getSelectedUniversityYears().map((year) => (
            <option key={year.year_code} value={year.year_code}>
              {year.year}년
            </option>
          ))}
        </select>
      </div>

      <div className="mb-6">
        <label className="block mb-2">카테고리 선택(중복가능)</label>
        <div className="space-y-2">
          {categories.map((category) => (
            <label key={category.id} className="flex items-center">
              <input
                type="checkbox"
                checked={selectedCategories.includes(category.id)}
                onChange={() => handleCategoryChange(category.id)}
                className="mr-2"
              />
              {category.name}
            </label>
          ))}
        </div>
      </div>

      {/* 디버깅을 위함 */}
      {/* <div className="text-sm text-gray-500 mb-4">
        선택된 카테고리 해시: {generateCategoryHash()}
      </div> */}

      {selectedSchool && selectedYear && selectedCategories.length > 0 && (
        <div className="mt-6">
          <h2 className="font-bold mb-2">구독 URL:</h2>
          <div className="p-3 bg-gray-100 rounded break-all">
            {generateUrl()}
          </div>
          <button
            className="mt-4 w-full bg-blue-500 text-white p-2 rounded hover:bg-blue-600"
            onClick={() => navigator.clipboard.writeText(generateUrl())}
          >
            URL 복사하기
          </button>
        </div>
      )}
    </div>
  );
}
