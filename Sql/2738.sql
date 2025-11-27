SELECT
  candidate.name,
  (
    ((score.math * 2) + (score.specific * 3) + (score.project_plan * 5))
    / 10
  )::NUMERIC(10, 2) as avg
FROM candidate
JOIN score ON score.candidate_id = candidate.id
ORDER BY avg DESC
